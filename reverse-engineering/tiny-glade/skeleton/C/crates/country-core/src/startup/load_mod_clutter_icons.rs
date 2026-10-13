// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/startup/load_mod_clutter_icons.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 1c76c4b00b0dc5545fc32bb1220586bf9973aec3e466894a9ca81a883292f41c
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: load_mod_clutter_icons.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 576, 1656, 1658, 1682
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.100"}]
// use clutter_icon_pipe; // namespace candidate
//   package identities: [{"name":"clutter-icon-pipe","source":"local_or_path","version":"0.1.0"}]
// use clutter_pack_pipe; // namespace candidate
//   package identities: [{"name":"clutter-pack-pipe","source":"local_or_path","version":"0.1.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use futures_lite; // namespace candidate
//   package identities: [{"name":"futures-lite","source":"registry+https://github.com/rust-lang/crates.io-index","version":"2.3.0"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// use tg_mods; // namespace candidate
//   package identities: [{"name":"tg-mods","source":"local_or_path","version":"0.1.0"}]
// use turbosloth; // namespace candidate
//   package identities: [{"name":"turbosloth","source":"local_or_path","version":"0.2.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_mod_clutter_icons::rebake_modded_clutter_icons
// 记录形式: compiled_procedure; instances=1
// 源码行: 25
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":576,"record_offset":15648,"rva":12133984,"va":5380843104}
// 编译实例 RVAs: 0xb92660
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::hash::random::RandomState>::new::KEYS::{K#0}::{closure#1}::__RUST_STD_INTERNAL_VAL::{shim:tls#0}
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t3801691","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380843104,"raw_hex":"e85b023a00","size":5,"soft_fail":false,"target_va":5384644912,"va":5380843216,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::time::Instant>::elapsed
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3798943","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380843104,"raw_hex":"e89ff73900","size":5,"soft_fail":false,"target_va":5384648448,"va":5380849500,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::time::Instant>::now
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3802375","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380843104,"raw_hex":"e807053a00","size":5,"soft_fail":false,"target_va":5384648032,"va":5380845652,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t3764743","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380843104,"raw_hex":"e807723900","size":5,"soft_fail":false,"target_va":5384609408,"va":5380844660,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-5165088","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380843104,"raw_hex":"e8e02fb1ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5380843771,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=36
//     evidence={"assembly":"\tcallq\t-5164912","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380843104,"raw_hex":"e89030b1ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5380843611,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=3
//     target_aliases=1699 candidates; complete list: load_mod_clutter_icons.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12121567","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380843104,"raw_hex":"e8210a47ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5380843754,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403ce4e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mod_clutter_icons.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8149071","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380843104,"raw_hex":"e8b1a783ff","size":5,"soft_fail":false,"target_va":5372699872,"va":5380848938,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14040fae0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: load_mod_clutter_icons.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7876276","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380843104,"raw_hex":"e84cd187ff","size":5,"soft_fail":false,"target_va":5372967648,"va":5380843919,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048d130
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: load_mod_clutter_icons.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7367656","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380843104,"raw_hex":"e818948fff","size":5,"soft_fail":false,"target_va":5373481264,"va":5380848915,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140523d10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mod_clutter_icons.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6750957","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380843104,"raw_hex":"e813fd98ff","size":5,"soft_fail":false,"target_va":5374098704,"va":5380849656,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140698b50
//     owner=unresolved_namespace; count=1
//     target_aliases=10 candidates; complete list: load_mod_clutter_icons.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5221575","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380843104,"raw_hex":"e83953b0ff","size":5,"soft_fail":false,"target_va":5375626064,"va":5380847634,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096c730
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_mod_clutter_icons.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2254973","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380843104,"raw_hex":"e88397ddff","size":5,"soft_fail":false,"target_va":5378590512,"va":5380845480,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096d120
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: load_mod_clutter_icons.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2257412","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380843104,"raw_hex":"e8fc8dddff","size":5,"soft_fail":false,"target_va":5378593056,"va":5380850463,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14098f4c0
//     owner=unresolved_namespace; count=3
//     target_aliases=5 candidates; complete list: load_mod_clutter_icons.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2118299","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380843104,"raw_hex":"e865addfff","size":5,"soft_fail":false,"target_va":5378733248,"va":5380851542,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14098f9c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mod_clutter_icons.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2116033","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380843104,"raw_hex":"e83fb6dfff","size":5,"soft_fail":false,"target_va":5378734528,"va":5380850556,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140990f00
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: load_mod_clutter_icons.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2111940","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380843104,"raw_hex":"e83cc6dfff","size":5,"soft_fail":false,"target_va":5378739968,"va":5380851903,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14099e0f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mod_clutter_icons.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2058296","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380843104,"raw_hex":"e8c897e0ff","size":5,"soft_fail":false,"target_va":5378793712,"va":5380852003,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409a5e30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mod_clutter_icons.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2025282","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380843104,"raw_hex":"e8be18e1ff","size":5,"soft_fail":false,"target_va":5378825776,"va":5380851053,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416f4de0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mod_clutter_icons.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11930504","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380843104,"raw_hex":"e8880bb600","size":5,"soft_fail":false,"target_va":5392780768,"va":5380850259,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416fe4b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mod_clutter_icons.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11973454","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380843104,"raw_hex":"e84eb3b600","size":5,"soft_fail":false,"target_va":5392819376,"va":5380845917,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141a86a40
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_mod_clutter_icons.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15677649","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380843104,"raw_hex":"e8d138ef00","size":5,"soft_fail":false,"target_va":5396523584,"va":5380845930,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b0dbc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mod_clutter_icons.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16230039","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380843104,"raw_hex":"e897a6f700","size":5,"soft_fail":false,"target_va":5397076928,"va":5380846884,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b0e380
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mod_clutter_icons.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16231178","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380843104,"raw_hex":"e80aabf700","size":5,"soft_fail":false,"target_va":5397078912,"va":5380847729,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d77a00
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: load_mod_clutter_icons.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35535340","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380843104,"raw_hex":"e8ec391e02","size":5,"soft_fail":false,"target_va":5416385024,"va":5380849679,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d9fd20
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: load_mod_clutter_icons.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35700467","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380843104,"raw_hex":"e8f3be2002","size":5,"soft_fail":false,"target_va":5416549664,"va":5380849192,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142dd38d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mod_clutter_icons.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35913772","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380843104,"raw_hex":"e82c002402","size":5,"soft_fail":false,"target_va":5416761552,"va":5380847775,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142dd56b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_mod_clutter_icons.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35922282","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380843104,"raw_hex":"e86a212402","size":5,"soft_fail":false,"target_va":5416769200,"va":5380846913,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: load_mod_clutter_icons.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36209335","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380843104,"raw_hex":"e8b7822802","size":5,"soft_fail":false,"target_va":5417057040,"va":5380847700,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: load_mod_clutter_icons.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36213246","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380843104,"raw_hex":"e8fe912802","size":5,"soft_fail":false,"target_va":5417057056,"va":5380843805,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t3950991","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380843104,"raw_hex":"e88f493c00","size":5,"soft_fail":false,"target_va":5384794864,"va":5380843868,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35537668","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380843104,"raw_hex":"e804431e02","size":5,"soft_fail":false,"target_va":5416381904,"va":5380844231,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t35773264","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380843104,"raw_hex":"e850db2102","size":5,"soft_fail":false,"target_va":5416620794,"va":5380847525,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3777708","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380843104,"raw_hex":"e8aca43900","size":5,"soft_fail":false,"target_va":5384624080,"va":5380846367,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] clutter_icon_pipe::icon_sources
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"clutter-icon-pipe","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1419565","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380843104,"raw_hex":"e82da91500","size":5,"soft_fail":false,"target_va":5382262880,"va":5380843310,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] clutter_pack_pipe::mod_icons::mod_clutter_icon_package_dir
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"clutter-pack-pipe","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1401723","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380843104,"raw_hex":"e87b631500","size":5,"soft_fail":false,"target_va":5382248592,"va":5380846864,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t35771670","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380843104,"raw_hex":"e816d52102","size":5,"soft_fail":false,"target_va":5416621408,"va":5380849733,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t19104","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380843104,"raw_hex":"e8a04a0000","size":5,"soft_fail":false,"target_va":5380864096,"va":5380844987,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t35771436","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380843104,"raw_hex":"e82cd42102","size":5,"soft_fail":false,"target_va":5416621376,"va":5380849935,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::texture_library::AssetTextureLibrary::add_or_replace
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1572158","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380843104,"raw_hex":"e8c202e8ff","size":5,"soft_fail":false,"target_va":5379276736,"va":5380848889,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::load_texture::load_packaged_texture_image
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-12191","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380843104,"raw_hex":"e861d0ffff","size":5,"soft_fail":false,"target_va":5380836512,"va":5380848698,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::startup::load_textures::reload_packaged_textures_by_name
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1031286","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380843104,"raw_hex":"e88a43f0ff","size":5,"soft_fail":false,"target_va":5379813424,"va":5380844705,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3118178","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380843104,"raw_hex":"e862942f00","size":5,"soft_fail":false,"target_va":5383963792,"va":5380845609,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3126686","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380843104,"raw_hex":"e89eb52f00","size":5,"soft_fail":false,"target_va":5383972224,"va":5380845533,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3118242","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380843104,"raw_hex":"e8a2942f00","size":5,"soft_fail":false,"target_va":5383964912,"va":5380846665,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3117871","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380843104,"raw_hex":"e82f932f00","size":5,"soft_fail":false,"target_va":5383962816,"va":5380844940,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::random::hashmap_random_keys
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t3877571","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380843104,"raw_hex":"e8c32a3b00","size":5,"soft_fail":false,"target_va":5384726688,"va":5380849112,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t35769647","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380843104,"raw_hex":"e82fcd2102","size":5,"soft_fail":false,"target_va":5416619360,"va":5380849708,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_mods::overlay::ChangedPaths::contains
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2042055","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380843104,"raw_hex":"e8c7281f00","size":5,"soft_fail":false,"target_va":5382885632,"va":5380843572,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_mods::overlay::is_mod_file
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2040740","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380843104,"raw_hex":"e8a4231f00","size":5,"soft_fail":false,"target_va":5382884432,"va":5380843687,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_mods::overlay::resolve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2075990","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380843104,"raw_hex":"e856ad1f00","size":5,"soft_fail":false,"target_va":5382919648,"va":5380843653,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":576,"record_offset":17968,"parent_record_offset":15648,"ancestor_record_offset":15648,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":576,"record_offset":17312,"parent_record_offset":15648,"ancestor_record_offset":15648,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":576,"record_offset":27632,"parent_record_offset":15648,"ancestor_record_offset":15648,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":576,"record_offset":20964,"parent_record_offset":15648,"ancestor_record_offset":15648,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":576,"record_offset":22492,"parent_record_offset":15648,"ancestor_record_offset":15648,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":576,"record_offset":15904,"parent_record_offset":15648,"ancestor_record_offset":15648,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":576,"record_offset":19164,"parent_record_offset":15648,"ancestor_record_offset":15648,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":576,"record_offset":16648,"parent_record_offset":15648,"ancestor_record_offset":15648,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":576,"record_offset":16764,"parent_record_offset":15648,"ancestor_record_offset":15648,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_direct] clutter_pack_pipe::mod_icons::build_mod_clutter_icons
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"clutter-pack-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/clutter-pack-pipe/src/mod_icons.rs
//     evidence={"module":576,"record_offset":27720,"parent_record_offset":15648,"ancestor_record_offset":15648,"inlinee":36611,"source_line":123,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":576,"record_offset":20796,"parent_record_offset":15648,"ancestor_record_offset":15648,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":576,"record_offset":18052,"parent_record_offset":15648,"ancestor_record_offset":15648,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":576,"record_offset":26264,"parent_record_offset":15648,"ancestor_record_offset":15648,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":576,"record_offset":18504,"parent_record_offset":15648,"ancestor_record_offset":15648,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":576,"record_offset":16916,"parent_record_offset":15648,"ancestor_record_offset":15648,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::load_mod_clutter_icons::rebake_modded_clutter_icons::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_mod_clutter_icons.rs
//     evidence={"module":576,"record_offset":33364,"parent_record_offset":15648,"ancestor_record_offset":15648,"inlinee":36613,"source_line":80,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_direct] futures_lite::future::block_on
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"futures-lite","version":"2.3.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/futures-lite-2.3.0/src/future.rs
//     evidence={"module":576,"record_offset":27748,"parent_record_offset":15648,"ancestor_record_offset":15648,"inlinee":5014,"source_line":56,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":576,"record_offset":20852,"parent_record_offset":15648,"ancestor_record_offset":15648,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":576,"record_offset":20712,"parent_record_offset":15648,"ancestor_record_offset":15648,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":576,"record_offset":25428,"parent_record_offset":15648,"ancestor_record_offset":15648,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":576,"record_offset":22520,"parent_record_offset":15648,"ancestor_record_offset":15648,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":576,"record_offset":22912,"parent_record_offset":15648,"ancestor_record_offset":15648,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::insert
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":576,"record_offset":18080,"parent_record_offset":15648,"ancestor_record_offset":15648,"inlinee":5646,"source_line":995,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":576,"record_offset":20628,"parent_record_offset":15648,"ancestor_record_offset":15648,"inlinee":26554,"source_line":385,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":576,"record_offset":15932,"parent_record_offset":15648,"ancestor_record_offset":15648,"inlinee":5639,"source_line":149,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":576,"record_offset":17788,"parent_record_offset":17764,"ancestor_record_offset":15648,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":576,"record_offset":17812,"parent_record_offset":17788,"ancestor_record_offset":15648,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=37
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":576,"record_offset":17188,"parent_record_offset":17164,"ancestor_record_offset":15648,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=37
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":576,"record_offset":17212,"parent_record_offset":17188,"ancestor_record_offset":15648,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":576,"record_offset":17836,"parent_record_offset":17812,"ancestor_record_offset":15648,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":576,"record_offset":17236,"parent_record_offset":17212,"ancestor_record_offset":15648,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":576,"record_offset":17764,"parent_record_offset":17608,"ancestor_record_offset":15648,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=37
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":576,"record_offset":17164,"parent_record_offset":17108,"ancestor_record_offset":15648,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":576,"record_offset":27392,"parent_record_offset":27368,"ancestor_record_offset":15648,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":576,"record_offset":18016,"parent_record_offset":17992,"ancestor_record_offset":15648,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":576,"record_offset":19268,"parent_record_offset":19236,"ancestor_record_offset":15648,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":576,"record_offset":17532,"parent_record_offset":17496,"ancestor_record_offset":15648,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=29
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":576,"record_offset":17136,"parent_record_offset":17108,"ancestor_record_offset":15648,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=29
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":576,"record_offset":17108,"parent_record_offset":17084,"ancestor_record_offset":15648,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":576,"record_offset":19316,"parent_record_offset":19292,"ancestor_record_offset":15648,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":576,"record_offset":19292,"parent_record_offset":19268,"ancestor_record_offset":15648,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":576,"record_offset":17608,"parent_record_offset":17568,"ancestor_record_offset":15648,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":576,"record_offset":17568,"parent_record_offset":17532,"ancestor_record_offset":15648,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=29
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":576,"record_offset":17084,"parent_record_offset":17060,"ancestor_record_offset":15648,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":576,"record_offset":17652,"parent_record_offset":17608,"ancestor_record_offset":15648,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":576,"record_offset":17384,"parent_record_offset":17348,"ancestor_record_offset":15648,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":576,"record_offset":17420,"parent_record_offset":17384,"ancestor_record_offset":15648,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":576,"record_offset":17456,"parent_record_offset":17420,"ancestor_record_offset":15648,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":576,"record_offset":17348,"parent_record_offset":17312,"ancestor_record_offset":15648,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":576,"record_offset":18568,"parent_record_offset":18540,"ancestor_record_offset":15648,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":576,"record_offset":35884,"parent_record_offset":35840,"ancestor_record_offset":15648,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$93::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":576,"record_offset":34264,"parent_record_offset":34096,"ancestor_record_offset":15648,"inlinee":21307,"source_line":2666,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":576,"record_offset":19236,"parent_record_offset":19192,"ancestor_record_offset":15648,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":576,"record_offset":19192,"parent_record_offset":19164,"ancestor_record_offset":15648,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":576,"record_offset":17496,"parent_record_offset":17456,"ancestor_record_offset":15648,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$18::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":576,"record_offset":35636,"parent_record_offset":33364,"ancestor_record_offset":15648,"inlinee":8387,"source_line":3913,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":576,"record_offset":34656,"parent_record_offset":34632,"ancestor_record_offset":15648,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":576,"record_offset":19812,"parent_record_offset":19788,"ancestor_record_offset":15648,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":576,"record_offset":20328,"parent_record_offset":20304,"ancestor_record_offset":15648,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::__private::format_err
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/lib.rs
//     evidence={"module":576,"record_offset":34572,"parent_record_offset":34548,"ancestor_record_offset":15648,"inlinee":7509,"source_line":687,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":576,"record_offset":17676,"parent_record_offset":17652,"ancestor_record_offset":15648,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":576,"record_offset":25784,"parent_record_offset":25756,"ancestor_record_offset":15648,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":576,"record_offset":16500,"parent_record_offset":16472,"ancestor_record_offset":15648,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":576,"record_offset":16400,"parent_record_offset":16196,"ancestor_record_offset":15648,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":576,"record_offset":25724,"parent_record_offset":25692,"ancestor_record_offset":15648,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":576,"record_offset":25756,"parent_record_offset":25724,"ancestor_record_offset":15648,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":576,"record_offset":25872,"parent_record_offset":25756,"ancestor_record_offset":15648,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":576,"record_offset":25984,"parent_record_offset":25960,"ancestor_record_offset":15648,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":576,"record_offset":34120,"parent_record_offset":34096,"ancestor_record_offset":15648,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":576,"record_offset":21312,"parent_record_offset":21288,"ancestor_record_offset":15648,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":576,"record_offset":17724,"parent_record_offset":17700,"ancestor_record_offset":15648,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":576,"record_offset":27236,"parent_record_offset":27212,"ancestor_record_offset":15648,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":576,"record_offset":27344,"parent_record_offset":27284,"ancestor_record_offset":15648,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":576,"record_offset":27212,"parent_record_offset":27084,"ancestor_record_offset":15648,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":576,"record_offset":27504,"parent_record_offset":27480,"ancestor_record_offset":15648,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\manually_drop.rs
//     evidence={"module":576,"record_offset":16676,"parent_record_offset":16648,"ancestor_record_offset":15648,"inlinee":5072,"source_line":184,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_dangling::MaybeDangling::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_dangling.rs
//     evidence={"module":576,"record_offset":16700,"parent_record_offset":16676,"ancestor_record_offset":15648,"inlinee":5074,"source_line":80,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":576,"record_offset":16364,"parent_record_offset":16340,"ancestor_record_offset":15648,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":576,"record_offset":17700,"parent_record_offset":17676,"ancestor_record_offset":15648,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":576,"record_offset":22268,"parent_record_offset":22240,"ancestor_record_offset":15648,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":576,"record_offset":27568,"parent_record_offset":27284,"ancestor_record_offset":15648,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":576,"record_offset":20456,"parent_record_offset":20428,"ancestor_record_offset":15648,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":576,"record_offset":16528,"parent_record_offset":16472,"ancestor_record_offset":15648,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":576,"record_offset":21532,"parent_record_offset":21508,"ancestor_record_offset":15648,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":576,"record_offset":16108,"parent_record_offset":16072,"ancestor_record_offset":15648,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":576,"record_offset":20820,"parent_record_offset":20796,"ancestor_record_offset":15648,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":576,"record_offset":34428,"parent_record_offset":33956,"ancestor_record_offset":15648,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":576,"record_offset":17992,"parent_record_offset":17968,"ancestor_record_offset":15648,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":576,"record_offset":34524,"parent_record_offset":33364,"ancestor_record_offset":15648,"inlinee":6793,"source_line":1363,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":576,"record_offset":16228,"parent_record_offset":16196,"ancestor_record_offset":15648,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::pin::Pin::new_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\pin.rs
//     evidence={"module":576,"record_offset":27796,"parent_record_offset":27748,"ancestor_record_offset":15648,"inlinee":5016,"source_line":1347,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":576,"record_offset":21344,"parent_record_offset":21260,"ancestor_record_offset":15648,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":576,"record_offset":26220,"parent_record_offset":25544,"ancestor_record_offset":15648,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":576,"record_offset":17892,"parent_record_offset":17456,"ancestor_record_offset":15648,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":576,"record_offset":17916,"parent_record_offset":17892,"ancestor_record_offset":15648,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=173
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":576,"record_offset":16940,"parent_record_offset":16916,"ancestor_record_offset":15648,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":576,"record_offset":16732,"parent_record_offset":16648,"ancestor_record_offset":15648,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":576,"record_offset":21800,"parent_record_offset":21776,"ancestor_record_offset":15648,"inlinee":4662,"source_line":1395,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":576,"record_offset":16316,"parent_record_offset":16196,"ancestor_record_offset":15648,"inlinee":5714,"source_line":1490,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":576,"record_offset":21652,"parent_record_offset":21628,"ancestor_record_offset":15648,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":576,"record_offset":16828,"parent_record_offset":16764,"ancestor_record_offset":15648,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":576,"record_offset":23336,"parent_record_offset":23312,"ancestor_record_offset":15648,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":576,"record_offset":16856,"parent_record_offset":16764,"ancestor_record_offset":15648,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":576,"record_offset":23312,"parent_record_offset":23288,"ancestor_record_offset":15648,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":576,"record_offset":16800,"parent_record_offset":16764,"ancestor_record_offset":15648,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":576,"record_offset":16880,"parent_record_offset":16856,"ancestor_record_offset":15648,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":576,"record_offset":16340,"parent_record_offset":16316,"ancestor_record_offset":15648,"inlinee":5715,"source_line":1542,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":576,"record_offset":19356,"parent_record_offset":19192,"ancestor_record_offset":15648,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":576,"record_offset":33484,"parent_record_offset":33364,"ancestor_record_offset":15648,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":576,"record_offset":33520,"parent_record_offset":33364,"ancestor_record_offset":15648,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":576,"record_offset":34192,"parent_record_offset":34168,"ancestor_record_offset":15648,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":576,"record_offset":23620,"parent_record_offset":23592,"ancestor_record_offset":15648,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":576,"record_offset":34220,"parent_record_offset":34192,"ancestor_record_offset":15648,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":576,"record_offset":23592,"parent_record_offset":23564,"ancestor_record_offset":15648,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":576,"record_offset":35660,"parent_record_offset":35636,"ancestor_record_offset":15648,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":576,"record_offset":35684,"parent_record_offset":35660,"ancestor_record_offset":15648,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":576,"record_offset":23264,"parent_record_offset":23240,"ancestor_record_offset":15648,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":576,"record_offset":23288,"parent_record_offset":23264,"ancestor_record_offset":15648,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":576,"record_offset":23648,"parent_record_offset":23620,"ancestor_record_offset":15648,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::find
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":576,"record_offset":33992,"parent_record_offset":33956,"ancestor_record_offset":15648,"inlinee":6791,"source_line":343,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":576,"record_offset":27284,"parent_record_offset":27260,"ancestor_record_offset":15648,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":576,"record_offset":34028,"parent_record_offset":33992,"ancestor_record_offset":15648,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":576,"record_offset":23240,"parent_record_offset":23124,"ancestor_record_offset":15648,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":576,"record_offset":22648,"parent_record_offset":22624,"ancestor_record_offset":15648,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":576,"record_offset":22948,"parent_record_offset":22912,"ancestor_record_offset":15648,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":576,"record_offset":23092,"parent_record_offset":23060,"ancestor_record_offset":15648,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":576,"record_offset":23060,"parent_record_offset":23020,"ancestor_record_offset":15648,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":576,"record_offset":22996,"parent_record_offset":22972,"ancestor_record_offset":15648,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":576,"record_offset":23020,"parent_record_offset":22996,"ancestor_record_offset":15648,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":576,"record_offset":22972,"parent_record_offset":22948,"ancestor_record_offset":15648,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":576,"record_offset":23564,"parent_record_offset":23020,"ancestor_record_offset":15648,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":576,"record_offset":22624,"parent_record_offset":22592,"ancestor_record_offset":15648,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":576,"record_offset":22592,"parent_record_offset":22564,"ancestor_record_offset":15648,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":576,"record_offset":22752,"parent_record_offset":22720,"ancestor_record_offset":15648,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":576,"record_offset":22808,"parent_record_offset":22752,"ancestor_record_offset":15648,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":576,"record_offset":22720,"parent_record_offset":22688,"ancestor_record_offset":15648,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":576,"record_offset":34144,"parent_record_offset":34120,"ancestor_record_offset":15648,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":576,"record_offset":22564,"parent_record_offset":22520,"ancestor_record_offset":15648,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":576,"record_offset":22832,"parent_record_offset":22808,"ancestor_record_offset":15648,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":576,"record_offset":23124,"parent_record_offset":23092,"ancestor_record_offset":15648,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":576,"record_offset":23408,"parent_record_offset":23124,"ancestor_record_offset":15648,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":576,"record_offset":23380,"parent_record_offset":23124,"ancestor_record_offset":15648,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":576,"record_offset":23184,"parent_record_offset":23124,"ancestor_record_offset":15648,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":576,"record_offset":27660,"parent_record_offset":27632,"ancestor_record_offset":15648,"inlinee":5322,"source_line":3134,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":576,"record_offset":20736,"parent_record_offset":20712,"ancestor_record_offset":15648,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":576,"record_offset":27684,"parent_record_offset":27660,"ancestor_record_offset":15648,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":576,"record_offset":20760,"parent_record_offset":20736,"ancestor_record_offset":15648,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::Asset::name
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":576,"record_offset":35840,"parent_record_offset":33364,"ancestor_record_offset":15648,"inlinee":21763,"source_line":34,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::Asset::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":576,"record_offset":35812,"parent_record_offset":33364,"ancestor_record_offset":15648,"inlinee":21762,"source_line":30,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::load_mod_clutter_icons::rebake_modded_clutter_icons::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_mod_clutter_icons.rs
//     evidence={"module":576,"record_offset":27368,"parent_record_offset":27344,"ancestor_record_offset":15648,"inlinee":36616,"source_line":64,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::load_mod_clutter_icons::rebake_modded_clutter_icons::closure$2::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_mod_clutter_icons.rs
//     evidence={"module":576,"record_offset":34548,"parent_record_offset":34524,"ancestor_record_offset":15648,"inlinee":36615,"source_line":87,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":576,"record_offset":21476,"parent_record_offset":21448,"ancestor_record_offset":15648,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":576,"record_offset":21508,"parent_record_offset":21476,"ancestor_record_offset":15648,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":576,"record_offset":21448,"parent_record_offset":21408,"ancestor_record_offset":15648,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=16
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":576,"record_offset":21288,"parent_record_offset":21260,"ancestor_record_offset":15648,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::is_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":576,"record_offset":20676,"parent_record_offset":20652,"ancestor_record_offset":15648,"inlinee":12123,"source_line":845,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":576,"record_offset":36584,"parent_record_offset":36560,"ancestor_record_offset":15648,"inlinee":4348,"source_line":2005,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove_entry
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":576,"record_offset":36612,"parent_record_offset":36584,"ancestor_record_offset":15648,"inlinee":4349,"source_line":2044,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::reserve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":576,"record_offset":27116,"parent_record_offset":27084,"ancestor_record_offset":15648,"inlinee":8392,"source_line":1103,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$82::extend
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":576,"record_offset":27084,"parent_record_offset":27056,"ancestor_record_offset":15648,"inlinee":12036,"source_line":4649,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$82::extend::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":576,"record_offset":27528,"parent_record_offset":27504,"ancestor_record_offset":15648,"inlinee":12038,"source_line":4661,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":576,"record_offset":36640,"parent_record_offset":36612,"ancestor_record_offset":15648,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::drop
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":576,"record_offset":21776,"parent_record_offset":21172,"ancestor_record_offset":15648,"inlinee":4661,"source_line":518,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=16
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":576,"record_offset":21628,"parent_record_offset":21408,"ancestor_record_offset":15648,"inlinee":4660,"source_line":490,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":576,"record_offset":21260,"parent_record_offset":21204,"ancestor_record_offset":15648,"inlinee":4653,"source_line":3584,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":576,"record_offset":21408,"parent_record_offset":21380,"ancestor_record_offset":15648,"inlinee":4659,"source_line":3651,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::reserve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":576,"record_offset":27144,"parent_record_offset":27116,"ancestor_record_offset":15648,"inlinee":8393,"source_line":955,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":576,"record_offset":22180,"parent_record_offset":22148,"ancestor_record_offset":15648,"inlinee":4366,"source_line":3160,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":576,"record_offset":21232,"parent_record_offset":21204,"ancestor_record_offset":15648,"inlinee":4651,"source_line":2478,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_elements
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":576,"record_offset":21172,"parent_record_offset":21112,"ancestor_record_offset":15648,"inlinee":4649,"source_line":2250,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":576,"record_offset":21112,"parent_record_offset":21088,"ancestor_record_offset":15648,"inlinee":4363,"source_line":2311,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":576,"record_offset":22148,"parent_record_offset":21112,"ancestor_record_offset":15648,"inlinee":4365,"source_line":3130,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":576,"record_offset":21144,"parent_record_offset":21112,"ancestor_record_offset":15648,"inlinee":4364,"source_line":2696,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":576,"record_offset":21204,"parent_record_offset":21172,"ancestor_record_offset":15648,"inlinee":4650,"source_line":2178,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":576,"record_offset":22212,"parent_record_offset":22180,"ancestor_record_offset":15648,"inlinee":4368,"source_line":218,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":576,"record_offset":21088,"parent_record_offset":21064,"ancestor_record_offset":15648,"inlinee":4648,"source_line":3493,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":576,"record_offset":21380,"parent_record_offset":21172,"ancestor_record_offset":15648,"inlinee":4658,"source_line":3848,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::insert
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":576,"record_offset":18104,"parent_record_offset":18080,"ancestor_record_offset":15648,"inlinee":5647,"source_line":1092,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::is_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":576,"record_offset":20652,"parent_record_offset":20628,"ancestor_record_offset":15648,"inlinee":26555,"source_line":326,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::remove
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":576,"record_offset":36560,"parent_record_offset":36536,"ancestor_record_offset":15648,"inlinee":13817,"source_line":1175,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$10::extend
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":576,"record_offset":27056,"parent_record_offset":27028,"ancestor_record_offset":15648,"inlinee":12034,"source_line":1307,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":576,"record_offset":20900,"parent_record_offset":20876,"ancestor_record_offset":15648,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":576,"record_offset":20876,"parent_record_offset":20852,"ancestor_record_offset":15648,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":576,"record_offset":20924,"parent_record_offset":20900,"ancestor_record_offset":15648,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":576,"record_offset":25464,"parent_record_offset":25428,"ancestor_record_offset":15648,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":576,"record_offset":31200,"parent_record_offset":31168,"ancestor_record_offset":15648,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":576,"record_offset":31680,"parent_record_offset":31464,"ancestor_record_offset":15648,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":576,"record_offset":25620,"parent_record_offset":25592,"ancestor_record_offset":15648,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":576,"record_offset":25692,"parent_record_offset":25544,"ancestor_record_offset":15648,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":576,"record_offset":25908,"parent_record_offset":25692,"ancestor_record_offset":15648,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::remove
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":576,"record_offset":36536,"parent_record_offset":33364,"ancestor_record_offset":15648,"inlinee":13816,"source_line":1042,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::with_hasher
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":576,"record_offset":27000,"parent_record_offset":26300,"ancestor_record_offset":15648,"inlinee":11864,"source_line":234,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$11::extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":576,"record_offset":27028,"parent_record_offset":26300,"ancestor_record_offset":15648,"inlinee":11866,"source_line":1200,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$13::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":576,"record_offset":15960,"parent_record_offset":15932,"ancestor_record_offset":15648,"inlinee":5641,"source_line":1246,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$9::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":576,"record_offset":26300,"parent_record_offset":26264,"ancestor_record_offset":15648,"inlinee":11863,"source_line":1149,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":576,"record_offset":16016,"parent_record_offset":15988,"ancestor_record_offset":15648,"inlinee":5645,"source_line":55,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__rust_std_internal_init_fn
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":576,"record_offset":16276,"parent_record_offset":16252,"ancestor_record_offset":15648,"inlinee":5713,"source_line":85,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":576,"record_offset":16136,"parent_record_offset":16108,"ancestor_record_offset":15648,"inlinee":5705,"source_line":100,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":576,"record_offset":16472,"parent_record_offset":16072,"ancestor_record_offset":15648,"inlinee":5709,"source_line":71,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":576,"record_offset":15988,"parent_record_offset":15960,"ancestor_record_offset":15648,"inlinee":5643,"source_line":149,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":576,"record_offset":16164,"parent_record_offset":16136,"ancestor_record_offset":15648,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init_slow
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":576,"record_offset":16196,"parent_record_offset":16164,"ancestor_record_offset":15648,"inlinee":5711,"source_line":69,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":576,"record_offset":16072,"parent_record_offset":16044,"ancestor_record_offset":15648,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":576,"record_offset":16044,"parent_record_offset":16016,"ancestor_record_offset":15648,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] turbosloth::store::PackageManifest::asset
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/store.rs
//     evidence={"module":576,"record_offset":33956,"parent_record_offset":33364,"ancestor_record_offset":15648,"inlinee":30962,"source_line":165,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] turbosloth::store::impl$6::asset::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/store.rs
//     evidence={"module":576,"record_offset":34096,"parent_record_offset":33992,"ancestor_record_offset":15648,"inlinee":30964,"source_line":168,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_mod_clutter_icons::rebake_modded_clutter_icons::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 64
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8F08
// 首条位置: {"module":576,"inlinee":36616}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":576,"record_offset":27392,"parent_record_offset":27368,"ancestor_record_offset":27368,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":576,"record_offset":27440,"parent_record_offset":27416,"ancestor_record_offset":27368,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":576,"record_offset":27416,"parent_record_offset":27392,"ancestor_record_offset":27368,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_mod_clutter_icons::rebake_modded_clutter_icons::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 80
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8F05
// 首条位置: {"module":576,"inlinee":36613}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$18::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":576,"record_offset":35636,"parent_record_offset":33364,"ancestor_record_offset":33364,"inlinee":8387,"source_line":3913,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::ok_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":576,"record_offset":34524,"parent_record_offset":33364,"ancestor_record_offset":33364,"inlinee":6793,"source_line":1363,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":576,"record_offset":33548,"parent_record_offset":33364,"ancestor_record_offset":33364,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":576,"record_offset":33484,"parent_record_offset":33364,"ancestor_record_offset":33364,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":576,"record_offset":33520,"parent_record_offset":33364,"ancestor_record_offset":33364,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":576,"record_offset":35748,"parent_record_offset":33364,"ancestor_record_offset":33364,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::Asset::name
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":576,"record_offset":35840,"parent_record_offset":33364,"ancestor_record_offset":33364,"inlinee":21763,"source_line":34,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::Asset::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":576,"record_offset":35812,"parent_record_offset":33364,"ancestor_record_offset":33364,"inlinee":21762,"source_line":30,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::remove
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":576,"record_offset":36536,"parent_record_offset":33364,"ancestor_record_offset":33364,"inlinee":13816,"source_line":1042,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_direct] turbosloth::store::PackageManifest::asset
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/store.rs
//     evidence={"module":576,"record_offset":33956,"parent_record_offset":33364,"ancestor_record_offset":33364,"inlinee":30962,"source_line":165,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":576,"record_offset":36320,"parent_record_offset":36296,"ancestor_record_offset":33364,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":576,"record_offset":36344,"parent_record_offset":36320,"ancestor_record_offset":33364,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":576,"record_offset":33836,"parent_record_offset":33812,"ancestor_record_offset":33364,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":576,"record_offset":33860,"parent_record_offset":33836,"ancestor_record_offset":33364,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":576,"record_offset":36368,"parent_record_offset":36344,"ancestor_record_offset":33364,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":576,"record_offset":33884,"parent_record_offset":33860,"ancestor_record_offset":33364,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":576,"record_offset":36296,"parent_record_offset":36148,"ancestor_record_offset":33364,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":576,"record_offset":33812,"parent_record_offset":33744,"ancestor_record_offset":33364,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":576,"record_offset":36088,"parent_record_offset":36060,"ancestor_record_offset":33364,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":576,"record_offset":33780,"parent_record_offset":33744,"ancestor_record_offset":33364,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":576,"record_offset":33744,"parent_record_offset":33716,"ancestor_record_offset":33364,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":576,"record_offset":36148,"parent_record_offset":36116,"ancestor_record_offset":33364,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":576,"record_offset":36116,"parent_record_offset":36088,"ancestor_record_offset":33364,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":576,"record_offset":33716,"parent_record_offset":33688,"ancestor_record_offset":33364,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":576,"record_offset":36184,"parent_record_offset":36148,"ancestor_record_offset":33364,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":576,"record_offset":35968,"parent_record_offset":35940,"ancestor_record_offset":33364,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":576,"record_offset":35996,"parent_record_offset":35968,"ancestor_record_offset":33364,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":576,"record_offset":36024,"parent_record_offset":35996,"ancestor_record_offset":33364,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":576,"record_offset":35940,"parent_record_offset":35912,"ancestor_record_offset":33364,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":576,"record_offset":35912,"parent_record_offset":35884,"ancestor_record_offset":33364,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":576,"record_offset":35884,"parent_record_offset":35840,"ancestor_record_offset":33364,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$93::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":576,"record_offset":34264,"parent_record_offset":34096,"ancestor_record_offset":33364,"inlinee":21307,"source_line":2666,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":576,"record_offset":36060,"parent_record_offset":36024,"ancestor_record_offset":33364,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":576,"record_offset":34656,"parent_record_offset":34632,"ancestor_record_offset":33364,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::__private::format_err
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/lib.rs
//     evidence={"module":576,"record_offset":34572,"parent_record_offset":34548,"ancestor_record_offset":33364,"inlinee":7509,"source_line":687,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":576,"record_offset":36208,"parent_record_offset":36184,"ancestor_record_offset":33364,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":576,"record_offset":34120,"parent_record_offset":34096,"ancestor_record_offset":33364,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":576,"record_offset":36256,"parent_record_offset":36232,"ancestor_record_offset":33364,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":576,"record_offset":36232,"parent_record_offset":36208,"ancestor_record_offset":33364,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":576,"record_offset":35472,"parent_record_offset":35444,"ancestor_record_offset":33364,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":576,"record_offset":34428,"parent_record_offset":33956,"ancestor_record_offset":33364,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":576,"record_offset":36424,"parent_record_offset":36024,"ancestor_record_offset":33364,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":576,"record_offset":36448,"parent_record_offset":36424,"ancestor_record_offset":33364,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=48
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":576,"record_offset":33576,"parent_record_offset":33548,"ancestor_record_offset":33364,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":576,"record_offset":35708,"parent_record_offset":35684,"ancestor_record_offset":33364,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":576,"record_offset":34056,"parent_record_offset":34028,"ancestor_record_offset":33364,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":576,"record_offset":34192,"parent_record_offset":34168,"ancestor_record_offset":33364,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":576,"record_offset":34220,"parent_record_offset":34192,"ancestor_record_offset":33364,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":576,"record_offset":35660,"parent_record_offset":35636,"ancestor_record_offset":33364,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":576,"record_offset":35684,"parent_record_offset":35660,"ancestor_record_offset":33364,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::find
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":576,"record_offset":33992,"parent_record_offset":33956,"ancestor_record_offset":33364,"inlinee":6791,"source_line":343,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":576,"record_offset":34028,"parent_record_offset":33992,"ancestor_record_offset":33364,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":576,"record_offset":34144,"parent_record_offset":34120,"ancestor_record_offset":33364,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::load_mod_clutter_icons::rebake_modded_clutter_icons::closure$2::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_mod_clutter_icons.rs
//     evidence={"module":576,"record_offset":34548,"parent_record_offset":34524,"ancestor_record_offset":33364,"inlinee":36615,"source_line":87,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":576,"record_offset":36584,"parent_record_offset":36560,"ancestor_record_offset":33364,"inlinee":4348,"source_line":2005,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove_entry
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":576,"record_offset":36612,"parent_record_offset":36584,"ancestor_record_offset":33364,"inlinee":4349,"source_line":2044,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":576,"record_offset":36640,"parent_record_offset":36612,"ancestor_record_offset":33364,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::remove
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":576,"record_offset":36560,"parent_record_offset":36536,"ancestor_record_offset":33364,"inlinee":13817,"source_line":1175,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] turbosloth::store::impl$6::asset::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/store.rs
//     evidence={"module":576,"record_offset":34096,"parent_record_offset":33992,"ancestor_record_offset":33364,"inlinee":30964,"source_line":168,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_mod_clutter_icons::rebake_modded_clutter_icons::closure$2::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 87
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8F07
// 首条位置: {"module":576,"inlinee":36615}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] anyhow::__private::format_err
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/lib.rs
//     evidence={"module":576,"record_offset":34572,"parent_record_offset":34548,"ancestor_record_offset":34548,"inlinee":7509,"source_line":687,"archive":"_pdb/C/modules/00576.jsonl.gz"}
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
// 完整结构及原始类型记录: load_mod_clutter_icons.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: load_mod_clutter_icons.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: load_mod_clutter_icons.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 14; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)
// 候选参数类型: ["ref$<country_core::resources::clutter_packs::ClutterPackObject>"]
// [文件级接口候选，具体函数归属 unknown] usize (*)(ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >)
// 候选参数类型: ["ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >"]
// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >,ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >)
// 候选参数类型: ["ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >", "ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >"]
// [文件级接口候选，具体函数归属 unknown] f32 (*)(f32,f32)
// 候选参数类型: ["f32", "f32"]
// [文件级接口候选，具体函数归属 unknown] void (*)(core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> >)
// 候选参数类型: ["core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> >"]
// [文件级接口候选，具体函数归属 unknown] ref$<str$> (*)(ref$<alloc::string::String>)
// 候选参数类型: ["ref$<alloc::string::String>"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<std::fs::DirEntry> > (*)(enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >)
// 候选参数类型: ["enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::ops::control_flow::ControlFlow<tuple$<ref$<country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::ui_wall_transform_gizmos::WallWireMetrics>,tuple$<> > > (*)(tuple$<ref$<country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::ui_wall_transform_gizmos::WallWireMetrics>)
// 候选参数类型: ["tuple$<ref$<country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::ui_wall_transform_gizmos::WallWireMetrics>"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_mod_clutter_icons::rebake_modded_clutter_icons
// 信息 ID: 5c2e8a96000843875b3a006c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::load_mod_textures::reload_mod_textures @ 0x9e4eb4 -> 0xb92660 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_mod_clutter_icons::rebake_modded_clutter_icons::closure$0
// 信息 ID: b14c1639e3b61afa8b805b58
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::startup::load_mod_clutter_icons::rebake_modded_clutter_icons", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::load_mod_clutter_icons::rebake_modded_clutter_icons; module=576, inline_record=27368; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_mod_clutter_icons::rebake_modded_clutter_icons::closure$2
// 信息 ID: 8318d30decd3f04bfd2df9b8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "country_core::startup::load_mod_clutter_icons::rebake_modded_clutter_icons", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::load_mod_clutter_icons::rebake_modded_clutter_icons; module=576, inline_record=33364; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_mod_clutter_icons::rebake_modded_clutter_icons::closure$2::closure$0
// 信息 ID: 82a1a49017e7c02a710f8eae
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "country_core::startup::load_mod_clutter_icons::rebake_modded_clutter_icons", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::load_mod_clutter_icons::rebake_modded_clutter_icons; module=576, inline_record=34548; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
