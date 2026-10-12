// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/tiny-glade/src/panic_reporter/panic_report.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: f41b246822a9ab784fa60b63fb739e9b01ebae58f722b8f5e1d1e4450bf46178
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: panic_report.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 81, 149, 189, 211, 230
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
// use lang_lib; // namespace candidate
//   package identities: [{"name":"lang-lib","source":"local_or_path","version":"0.1.0"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use sentry_core; // namespace candidate
//   package identities: [{"name":"sentry-core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.31.7"}]
// use sentry_types; // namespace candidate
//   package identities: [{"name":"sentry-types","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.31.7"}]
// use std; // namespace candidate
//   package identities: []
// use tg_package_integrity; // namespace candidate
//   package identities: [{"name":"tg-package-integrity","source":"local_or_path","version":"0.1.0"}]
// use tiny_glade; // namespace candidate
//   package identities: [{"name":"tiny-glade","source":"local_or_path","version":"0.2.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_report::on_game_crashed
// 记录形式: compiled_procedure; instances=1
// 源码行: 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":189,"record_offset":536,"rva":5512272,"va":5374221392}
// 编译实例 RVAs: 0x541c50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <alloc::string::String as core::clone::Clone>::clone
//     owner=unresolved_namespace; count=13
//     evidence={"assembly":"\tcallq\t10574260","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374221392,"raw_hex":"e8b459a100","size":5,"soft_fail":false,"target_va":5384795984,"va":5374221719,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::fs::DirBuilder>::_create
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10462860","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374221392,"raw_hex":"e88ca69f00","size":5,"soft_fail":false,"target_va":5384689616,"va":5374226751,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::_join
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t10432879","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374221392,"raw_hex":"e86f319f00","size":5,"soft_fail":false,"target_va":5384656240,"va":5374223356,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::to_path_buf
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10431789","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374221392,"raw_hex":"e82d2d9f00","size":5,"soft_fail":false,"target_va":5384653728,"va":5374221934,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=7
//     evidence={"assembly":"\tcallq\t10387751","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374221392,"raw_hex":"e827819e00","size":5,"soft_fail":false,"target_va":5384609408,"va":5374221652,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$tempfile..dir..TempDir$u20$as$u20$core..ops..drop..Drop$GT$::drop
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t7761669","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374221392,"raw_hex":"e8056f7600","size":5,"soft_fail":false,"target_va":5381991392,"va":5374229718,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=14
//     evidence={"assembly":"\tcallq\t1456706","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374221392,"raw_hex":"e8423a1600","size":5,"soft_fail":false,"target_va":5375678688,"va":5374221977,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=24
//     evidence={"assembly":"\tcallq\t1456949","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374221392,"raw_hex":"e8353b1600","size":5,"soft_fail":false,"target_va":5375678704,"va":5374221750,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=14
//     target_aliases=1699 candidates; complete list: panic_report.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5499771","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374221392,"raw_hex":"e88514acff","size":5,"soft_fail":false,"target_va":5368722192,"va":5374221958,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14007eec0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: panic_report.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5004467","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374221392,"raw_hex":"e84da3b3ff","size":5,"soft_fail":false,"target_va":5369228992,"va":5374233454,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140083cb0
//     owner=unresolved_namespace; count=1
//     target_aliases=131 candidates; complete list: panic_report.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4982799","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374221392,"raw_hex":"e8f1f7b3ff","size":5,"soft_fail":false,"target_va":5369248944,"va":5374231738,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14008e050
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: panic_report.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4941992","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374221392,"raw_hex":"e85897b4ff","size":5,"soft_fail":false,"target_va":5369290832,"va":5374232819,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140094fe0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: panic_report.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4912150","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374221392,"raw_hex":"e8ea0bb5ff","size":5,"soft_fail":false,"target_va":5369319392,"va":5374231537,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=6
//     target_aliases=3 candidates; complete list: panic_report.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4884607","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374221392,"raw_hex":"e88177b5ff","size":5,"soft_fail":false,"target_va":5369338848,"va":5374223450,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14009bbb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: panic_report.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4876590","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374221392,"raw_hex":"e8d296b5ff","size":5,"soft_fail":false,"target_va":5369346992,"va":5374223577,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b0540
//     owner=unresolved_namespace; count=3
//     target_aliases=26 candidates; complete list: panic_report.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4800077","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374221392,"raw_hex":"e8b3c1b6ff","size":5,"soft_fail":false,"target_va":5369431360,"va":5374231432,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b2470
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: panic_report.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4791352","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374221392,"raw_hex":"e8c8e3b6ff","size":5,"soft_fail":false,"target_va":5369439344,"va":5374230691,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b8670
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: panic_report.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4768946","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374221392,"raw_hex":"e84e3bb7ff","size":5,"soft_fail":false,"target_va":5369464432,"va":5374233373,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b9330
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: panic_report.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4765256","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374221392,"raw_hex":"e8b849b7ff","size":5,"soft_fail":false,"target_va":5369467696,"va":5374232947,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400bac40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: panic_report.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4752122","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374221392,"raw_hex":"e8067db7ff","size":5,"soft_fail":false,"target_va":5369474112,"va":5374226229,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140103f10
//     owner=unresolved_namespace; count=6
//     target_aliases=3 candidates; complete list: panic_report.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4457080","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374221392,"raw_hex":"e888fdbbff","size":5,"soft_fail":false,"target_va":5369773840,"va":5374230915,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14018be80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: panic_report.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3902367","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374221392,"raw_hex":"e86174c4ff","size":5,"soft_fail":false,"target_va":5370330752,"va":5374233114,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140221130
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: panic_report.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3287926","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374221392,"raw_hex":"e88ad4cdff","size":5,"soft_fail":false,"target_va":5370941744,"va":5374229665,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402fb580
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: panic_report.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2390037","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374221392,"raw_hex":"e8eb87dbff","size":5,"soft_fail":false,"target_va":5371835776,"va":5374225808,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402fb850
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: panic_report.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2386331","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374221392,"raw_hex":"e86596dbff","size":5,"soft_fail":false,"target_va":5371836496,"va":5374222822,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140300d40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: panic_report.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2369892","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374221392,"raw_hex":"e89cd6dbff","size":5,"soft_fail":false,"target_va":5371858240,"va":5374228127,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140523d10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: panic_report.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-127653","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374221392,"raw_hex":"e85b0dfeff","size":5,"soft_fail":false,"target_va":5374098704,"va":5374226352,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140545030
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: panic_report.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2523","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374221392,"raw_hex":"e8db090000","size":5,"soft_fail":false,"target_va":5374234672,"va":5374232144,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14059a030
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: panic_report.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t353882","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374221392,"raw_hex":"e85a660500","size":5,"soft_fail":false,"target_va":5374582832,"va":5374228945,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140caac10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: panic_report.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7769659","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374221392,"raw_hex":"e83b8e7600","size":5,"soft_fail":false,"target_va":5381991440,"va":5374221776,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=19
//     target_aliases=1 candidates; complete list: panic_report.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42835052","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374221392,"raw_hex":"e86c9c8d02","size":5,"soft_fail":false,"target_va":5417057056,"va":5374221999,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t42390748","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374221392,"raw_hex":"e8dcd48602","size":5,"soft_fail":false,"target_va":5416620769,"va":5374230016,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10567977","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374221392,"raw_hex":"e82941a100","size":5,"soft_fail":false,"target_va":5384794864,"va":5374226882,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=13
//     evidence={"assembly":"\tcallq\t42398904","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374221392,"raw_hex":"e8b8f48602","size":5,"soft_fail":false,"target_va":5416620794,"va":5374221885,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::__private::format_err
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t42138444","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374221392,"raw_hex":"e84cfb8202","size":5,"soft_fail":false,"target_va":5416360672,"va":5374222223,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10401294","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374221392,"raw_hex":"e80eb69e00","size":5,"soft_fail":false,"target_va":5384624080,"va":5374222781,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t42391358","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374221392,"raw_hex":"e83ed78602","size":5,"soft_fail":false,"target_va":5416621408,"va":5374230045,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<enum2$<core::result::Result<alloc::string::String,anyhow::Error> >,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2949","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374221392,"raw_hex":"e8850b0000","size":5,"soft_fail":false,"target_va":5374234880,"va":5374231926,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<tempfile::dir::TempDir,std::io::error::Error> > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1848","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374221392,"raw_hex":"e838070000","size":5,"soft_fail":false,"target_va":5374235104,"va":5374233251,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<tiny_glade::crash_pickup::crash_payload::RecoveredSave,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError> > > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-4081864","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374221392,"raw_hex":"e838b7c1ff","size":5,"soft_fail":false,"target_va":5370151808,"va":5374233667,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,enum2$<tg_package_integrity::PackageVerificationError> > > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1569","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374221392,"raw_hex":"e821060000","size":5,"soft_fail":false,"target_va":5374234256,"va":5374232682,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::load_last_used_locale
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8911642","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374221392,"raw_hex":"e81afb8700","size":5,"soft_fail":false,"target_va":5383134160,"va":5374222513,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::fs::canonicalize
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10499909","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374221392,"raw_hex":"e84537a000","size":5,"soft_fail":false,"target_va":5384722976,"va":5374223062,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::fs::copy
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t10500236","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374221392,"raw_hex":"e88c38a000","size":5,"soft_fail":false,"target_va":5384723728,"va":5374223487,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::fs::rename
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10501036","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374221392,"raw_hex":"e8ac3ba000","size":5,"soft_fail":false,"target_va":5384724448,"va":5374223407,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::lifecycle::JoinInner$LT$T$GT$::join
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-854787","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374221392,"raw_hex":"e8fdf4f2ff","size":5,"soft_fail":false,"target_va":5373374256,"va":5374229038,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_package_integrity::verify_package_manifest
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-package-integrity","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1940263","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374221392,"raw_hex":"e8279b1d00","size":5,"soft_fail":false,"target_va":5376167776,"va":5374227508,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-44490","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374221392,"raw_hex":"e83652ffff","size":5,"soft_fail":false,"target_va":5374182912,"va":5374227397,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$2::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t42137469","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374221392,"raw_hex":"e87df78202","size":5,"soft_fail":false,"target_va":5416360560,"va":5374223086,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::file::date_time_string_for_filename
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9073242","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374221392,"raw_hex":"e85a728a00","size":5,"soft_fail":false,"target_va":5383300064,"va":5374226817,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5374221392,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5374229330,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":189,"record_offset":15208,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_direct] alloc::rc::Rc::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\rc.rs
//     evidence={"module":189,"record_offset":18732,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":10201,"source_line":413,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_direct] alloc::rc::impl$28::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\rc.rs
//     evidence={"module":189,"record_offset":19224,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":12899,"source_line":2415,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":189,"record_offset":18520,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_direct] core::cell::RefCell::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":189,"record_offset":18704,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":14322,"source_line":971,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_direct] core::cell::RefCell::take
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":189,"record_offset":19308,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":15608,"source_line":1430,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":189,"record_offset":924,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":189,"record_offset":15292,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":189,"record_offset":18356,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":189,"record_offset":784,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":4877,"source_line":684,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":189,"record_offset":18552,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":5839,"source_line":1224,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":189,"record_offset":1828,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":189,"record_offset":2332,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":7484,"source_line":1488,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":189,"record_offset":2140,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":8050,"source_line":768,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::is_ok
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":189,"record_offset":13920,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":5676,"source_line":593,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":189,"record_offset":2168,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":189,"record_offset":2260,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":189,"record_offset":17968,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":5287,"source_line":708,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$5::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":189,"record_offset":1092,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":15676,"source_line":1893,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":189,"record_offset":980,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":189,"record_offset":840,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_direct] std::fs::copy
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":189,"record_offset":13976,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":8451,"source_line":2880,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_direct] std::fs::create_dir_all
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":189,"record_offset":15068,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":7539,"source_line":3123,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_direct] std::fs::rename
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":189,"record_offset":13836,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":13844,"source_line":2818,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":189,"record_offset":13808,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_direct] std::thread::join_handle::JoinHandle::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\join_handle.rs
//     evidence={"module":189,"record_offset":19640,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":12195,"source_line":149,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::crash_pickup::crash_payload::impl$7::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":189,"record_offset":3772,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":15683,"source_line":26,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":189,"record_offset":1636,"parent_record_offset":1612,"ancestor_record_offset":536,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":189,"record_offset":1660,"parent_record_offset":1636,"ancestor_record_offset":536,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":189,"record_offset":2028,"parent_record_offset":2004,"ancestor_record_offset":536,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":189,"record_offset":2052,"parent_record_offset":2028,"ancestor_record_offset":536,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":189,"record_offset":1684,"parent_record_offset":1660,"ancestor_record_offset":536,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":189,"record_offset":2076,"parent_record_offset":2052,"ancestor_record_offset":536,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":189,"record_offset":1612,"parent_record_offset":1460,"ancestor_record_offset":536,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":189,"record_offset":2004,"parent_record_offset":1948,"ancestor_record_offset":536,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":189,"record_offset":18812,"parent_record_offset":18768,"ancestor_record_offset":536,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":189,"record_offset":18768,"parent_record_offset":18732,"ancestor_record_offset":536,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":189,"record_offset":19948,"parent_record_offset":19920,"ancestor_record_offset":536,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":189,"record_offset":15256,"parent_record_offset":15232,"ancestor_record_offset":536,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":189,"record_offset":1396,"parent_record_offset":1368,"ancestor_record_offset":536,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":189,"record_offset":1976,"parent_record_offset":1948,"ancestor_record_offset":536,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":189,"record_offset":1948,"parent_record_offset":1924,"ancestor_record_offset":536,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":189,"record_offset":1460,"parent_record_offset":1424,"ancestor_record_offset":536,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":189,"record_offset":1424,"parent_record_offset":1396,"ancestor_record_offset":536,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":189,"record_offset":1924,"parent_record_offset":1900,"ancestor_record_offset":536,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":189,"record_offset":1500,"parent_record_offset":1460,"ancestor_record_offset":536,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::rc::Rc::inner
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\rc.rs
//     evidence={"module":189,"record_offset":19248,"parent_record_offset":19224,"ancestor_record_offset":536,"inlinee":4675,"source_line":363,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::rc::impl$35::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\rc.rs
//     evidence={"module":189,"record_offset":21096,"parent_record_offset":21072,"ancestor_record_offset":536,"inlinee":4673,"source_line":2468,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":189,"record_offset":6648,"parent_record_offset":6572,"ancestor_record_offset":536,"inlinee":8468,"source_line":410,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":189,"record_offset":1336,"parent_record_offset":1272,"ancestor_record_offset":536,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":189,"record_offset":1308,"parent_record_offset":1272,"ancestor_record_offset":536,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":189,"record_offset":1368,"parent_record_offset":1336,"ancestor_record_offset":536,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":189,"record_offset":1244,"parent_record_offset":1216,"ancestor_record_offset":536,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":189,"record_offset":18636,"parent_record_offset":18612,"ancestor_record_offset":536,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":189,"record_offset":1272,"parent_record_offset":1244,"ancestor_record_offset":536,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::wtf8::impl$14::clone
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\wtf8\mod.rs
//     evidence={"module":189,"record_offset":1216,"parent_record_offset":1188,"ancestor_record_offset":536,"inlinee":6712,"source_line":38,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":189,"record_offset":19980,"parent_record_offset":19948,"ancestor_record_offset":536,"inlinee":4689,"source_line":253,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":189,"record_offset":1524,"parent_record_offset":1500,"ancestor_record_offset":536,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":189,"record_offset":19428,"parent_record_offset":19404,"ancestor_record_offset":536,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":189,"record_offset":19564,"parent_record_offset":19540,"ancestor_record_offset":536,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":189,"record_offset":21180,"parent_record_offset":21124,"ancestor_record_offset":536,"inlinee":4677,"source_line":433,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":189,"record_offset":19372,"parent_record_offset":19340,"ancestor_record_offset":536,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":189,"record_offset":19340,"parent_record_offset":19308,"ancestor_record_offset":536,"inlinee":15583,"source_line":1025,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":189,"record_offset":19404,"parent_record_offset":19372,"ancestor_record_offset":536,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":189,"record_offset":19540,"parent_record_offset":19516,"ancestor_record_offset":536,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":189,"record_offset":13860,"parent_record_offset":13836,"ancestor_record_offset":536,"inlinee":5107,"source_line":717,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":189,"record_offset":1572,"parent_record_offset":1548,"ancestor_record_offset":536,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":189,"record_offset":7172,"parent_record_offset":7128,"ancestor_record_offset":536,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::take::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\take.rs
//     evidence={"module":189,"record_offset":7128,"parent_record_offset":6648,"ancestor_record_offset":536,"inlinee":8469,"source_line":36,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":189,"record_offset":18612,"parent_record_offset":18588,"ancestor_record_offset":536,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::align_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":189,"record_offset":20044,"parent_record_offset":20008,"ancestor_record_offset":536,"inlinee":4691,"source_line":566,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":189,"record_offset":7264,"parent_record_offset":6648,"ancestor_record_offset":536,"inlinee":4509,"source_line":563,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":189,"record_offset":19464,"parent_record_offset":19340,"ancestor_record_offset":536,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":189,"record_offset":20008,"parent_record_offset":19980,"ancestor_record_offset":536,"inlinee":4690,"source_line":427,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":189,"record_offset":1548,"parent_record_offset":1524,"ancestor_record_offset":536,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":189,"record_offset":808,"parent_record_offset":784,"ancestor_record_offset":536,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":189,"record_offset":948,"parent_record_offset":924,"ancestor_record_offset":536,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":189,"record_offset":15232,"parent_record_offset":15208,"ancestor_record_offset":536,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$6::clone
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":189,"record_offset":6276,"parent_record_offset":5500,"ancestor_record_offset":536,"inlinee":5739,"source_line":2261,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":189,"record_offset":1740,"parent_record_offset":1336,"ancestor_record_offset":536,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":189,"record_offset":1764,"parent_record_offset":1740,"ancestor_record_offset":536,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=97
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":189,"record_offset":1852,"parent_record_offset":1828,"ancestor_record_offset":536,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":189,"record_offset":19272,"parent_record_offset":19248,"ancestor_record_offset":536,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":189,"record_offset":7224,"parent_record_offset":7200,"ancestor_record_offset":536,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":189,"record_offset":2460,"parent_record_offset":2372,"ancestor_record_offset":536,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$5::clone
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":189,"record_offset":3944,"parent_record_offset":3772,"ancestor_record_offset":536,"inlinee":15676,"source_line":1893,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":189,"record_offset":18448,"parent_record_offset":18420,"ancestor_record_offset":536,"inlinee":5756,"source_line":281,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":189,"record_offset":7200,"parent_record_offset":7172,"ancestor_record_offset":536,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":189,"record_offset":864,"parent_record_offset":840,"ancestor_record_offset":536,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":189,"record_offset":888,"parent_record_offset":864,"ancestor_record_offset":536,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::grading_lut::impl$5::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/grading_lut.rs
//     evidence={"module":189,"record_offset":7372,"parent_record_offset":7336,"ancestor_record_offset":536,"inlinee":8473,"source_line":24,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::image_formation::impl$12::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/image_formation.rs
//     evidence={"module":189,"record_offset":7336,"parent_record_offset":7308,"ancestor_record_offset":536,"inlinee":8471,"source_line":27,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::image_formation::impl$2::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/image_formation.rs
//     evidence={"module":189,"record_offset":7416,"parent_record_offset":7336,"ancestor_record_offset":536,"inlinee":8475,"source_line":11,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::impl$20::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":189,"record_offset":7508,"parent_record_offset":7480,"ancestor_record_offset":536,"inlinee":8479,"source_line":122,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::impl$30::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":189,"record_offset":6464,"parent_record_offset":6412,"ancestor_record_offset":536,"inlinee":8466,"source_line":176,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::impl$31::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":189,"record_offset":6412,"parent_record_offset":6336,"ancestor_record_offset":536,"inlinee":8464,"source_line":278,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::import::impl$1::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session/import.rs
//     evidence={"module":189,"record_offset":7480,"parent_record_offset":7452,"ancestor_record_offset":536,"inlinee":8477,"source_line":17,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":189,"record_offset":1028,"parent_record_offset":1004,"ancestor_record_offset":536,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":189,"record_offset":1004,"parent_record_offset":980,"ancestor_record_offset":536,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":189,"record_offset":1052,"parent_record_offset":1028,"ancestor_record_offset":536,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":189,"record_offset":1160,"parent_record_offset":1132,"ancestor_record_offset":536,"inlinee":6708,"source_line":699,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::create
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":189,"record_offset":15148,"parent_record_offset":15068,"ancestor_record_offset":536,"inlinee":7542,"source_line":3441,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::recursive
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":189,"record_offset":15092,"parent_record_offset":15068,"ancestor_record_offset":536,"inlinee":7541,"source_line":3417,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::canonicalize
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":189,"record_offset":2428,"parent_record_offset":2404,"ancestor_record_offset":536,"inlinee":8115,"source_line":3035,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::canonicalize
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":189,"record_offset":2404,"parent_record_offset":2372,"ancestor_record_offset":536,"inlinee":8114,"source_line":3307,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$30::clone
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":189,"record_offset":1132,"parent_record_offset":1092,"ancestor_record_offset":536,"inlinee":6706,"source_line":1874,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$59::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":189,"record_offset":2224,"parent_record_offset":2200,"ancestor_record_offset":536,"inlinee":15680,"source_line":2206,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$7::clone
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":189,"record_offset":1188,"parent_record_offset":1160,"ancestor_record_offset":536,"inlinee":6710,"source_line":70,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::impl$6::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":189,"record_offset":5464,"parent_record_offset":5392,"ancestor_record_offset":536,"inlinee":15685,"source_line":17,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::impl$8::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":189,"record_offset":5500,"parent_record_offset":5392,"ancestor_record_offset":536,"inlinee":15688,"source_line":39,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::impl$9::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":189,"record_offset":6336,"parent_record_offset":6276,"ancestor_record_offset":536,"inlinee":15690,"source_line":45,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/panic_report.rs
//     evidence={"module":189,"record_offset":2200,"parent_record_offset":2168,"ancestor_record_offset":536,"inlinee":15678,"source_line":47,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/panic_report.rs
//     evidence={"module":189,"record_offset":2300,"parent_record_offset":2260,"ancestor_record_offset":536,"inlinee":15681,"source_line":51,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/panic_report.rs
//     evidence={"module":189,"record_offset":2372,"parent_record_offset":2332,"ancestor_record_offset":536,"inlinee":15686,"source_line":52,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$4
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/panic_report.rs
//     evidence={"module":189,"record_offset":18420,"parent_record_offset":18384,"ancestor_record_offset":536,"inlinee":15691,"source_line":138,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$5
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/panic_report.rs
//     evidence={"module":189,"record_offset":18588,"parent_record_offset":18552,"ancestor_record_offset":536,"inlinee":15692,"source_line":139,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 47
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3D3E
// 首条位置: {"module":189,"inlinee":15678}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::path::impl$59::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":189,"record_offset":2224,"parent_record_offset":2200,"ancestor_record_offset":2200,"inlinee":15680,"source_line":2206,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 51
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3D41
// 首条位置: {"module":189,"inlinee":15681}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 52
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3D46
// 首条位置: {"module":189,"inlinee":15686}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":189,"record_offset":2496,"parent_record_offset":2372,"ancestor_record_offset":2372,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":189,"record_offset":2460,"parent_record_offset":2372,"ancestor_record_offset":2372,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::canonicalize
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":189,"record_offset":2404,"parent_record_offset":2372,"ancestor_record_offset":2372,"inlinee":8114,"source_line":3307,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":189,"record_offset":2744,"parent_record_offset":2720,"ancestor_record_offset":2372,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":189,"record_offset":2768,"parent_record_offset":2744,"ancestor_record_offset":2372,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":189,"record_offset":2792,"parent_record_offset":2768,"ancestor_record_offset":2372,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":189,"record_offset":2720,"parent_record_offset":2664,"ancestor_record_offset":2372,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":189,"record_offset":2692,"parent_record_offset":2664,"ancestor_record_offset":2372,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":189,"record_offset":2664,"parent_record_offset":2640,"ancestor_record_offset":2372,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":189,"record_offset":2640,"parent_record_offset":2616,"ancestor_record_offset":2372,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":189,"record_offset":2520,"parent_record_offset":2496,"ancestor_record_offset":2372,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::canonicalize
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":189,"record_offset":2428,"parent_record_offset":2404,"ancestor_record_offset":2372,"inlinee":8115,"source_line":3035,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$2::closure$0
// 记录形式: compiled_procedure; instances=1
// 源码行: 52
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":189,"record_offset":23228,"rva":47651440,"va":5416360560}
// 编译实例 RVAs: 0x2d71a70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: panic_report.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-47021776","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416360560,"raw_hex":"e8308132fd","size":5,"soft_fail":false,"target_va":5369338848,"va":5416360619,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::__private::format_err
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t61","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416360560,"raw_hex":"e83d000000","size":5,"soft_fail":false,"target_va":5416360672,"va":5416360606,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 138
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3D4B
// 首条位置: {"module":189,"inlinee":15691}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::impl$6::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":189,"record_offset":18476,"parent_record_offset":18420,"ancestor_record_offset":18420,"inlinee":5739,"source_line":2261,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":189,"record_offset":18448,"parent_record_offset":18420,"ancestor_record_offset":18420,"inlinee":5756,"source_line":281,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$5
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 139
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3D4C
// 首条位置: {"module":189,"inlinee":15692}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":189,"record_offset":18612,"parent_record_offset":18588,"ancestor_record_offset":18588,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":189,"record_offset":18636,"parent_record_offset":18612,"ancestor_record_offset":18588,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00189.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$5::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 143
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2AFA
// 首条位置: {"module":81,"inlinee":11002}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":81,"record_offset":149732,"parent_record_offset":149708,"ancestor_record_offset":149708,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$48::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":81,"record_offset":149760,"parent_record_offset":149708,"ancestor_record_offset":149708,"inlinee":10158,"source_line":2779,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$6
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 153
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x42E5
// 首条位置: {"module":230,"inlinee":17125}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":230,"record_offset":14584,"parent_record_offset":13596,"ancestor_record_offset":13596,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":230,"record_offset":13788,"parent_record_offset":13596,"ancestor_record_offset":13596,"inlinee":6896,"source_line":419,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":230,"record_offset":14768,"parent_record_offset":13596,"ancestor_record_offset":13596,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":230,"record_offset":14556,"parent_record_offset":13596,"ancestor_record_offset":13596,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":230,"record_offset":18592,"parent_record_offset":13596,"ancestor_record_offset":13596,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":230,"record_offset":14036,"parent_record_offset":13596,"ancestor_record_offset":13596,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":230,"record_offset":17888,"parent_record_offset":13596,"ancestor_record_offset":13596,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_direct] sentry_core::clientoptions::impl$2::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"sentry-core","version":"0.31.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sentry-core-0.31.7/src/clientoptions.rs
//     evidence={"module":230,"record_offset":14008,"parent_record_offset":13596,"ancestor_record_offset":13596,"inlinee":17127,"source_line":230,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_direct] sentry_types::protocol::v7::impl$172::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"sentry-types","version":"0.31.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sentry-types-0.31.7/src/protocol/v7.rs
//     evidence={"module":230,"record_offset":15832,"parent_record_offset":13596,"ancestor_record_offset":13596,"inlinee":10152,"source_line":619,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$30::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":230,"record_offset":16572,"parent_record_offset":13596,"ancestor_record_offset":13596,"inlinee":6706,"source_line":1874,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":230,"record_offset":13908,"parent_record_offset":13884,"ancestor_record_offset":13596,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":230,"record_offset":13932,"parent_record_offset":13908,"ancestor_record_offset":13596,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":230,"record_offset":14260,"parent_record_offset":14236,"ancestor_record_offset":13596,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":230,"record_offset":14284,"parent_record_offset":14260,"ancestor_record_offset":13596,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":230,"record_offset":13956,"parent_record_offset":13932,"ancestor_record_offset":13596,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":230,"record_offset":14308,"parent_record_offset":14284,"ancestor_record_offset":13596,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":230,"record_offset":13884,"parent_record_offset":13852,"ancestor_record_offset":13596,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":230,"record_offset":14236,"parent_record_offset":14180,"ancestor_record_offset":13596,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":230,"record_offset":13852,"parent_record_offset":13816,"ancestor_record_offset":13596,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":230,"record_offset":13816,"parent_record_offset":13788,"ancestor_record_offset":13596,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":230,"record_offset":19148,"parent_record_offset":19124,"ancestor_record_offset":13596,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":230,"record_offset":17944,"parent_record_offset":17920,"ancestor_record_offset":13596,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":230,"record_offset":17992,"parent_record_offset":17968,"ancestor_record_offset":13596,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":230,"record_offset":15000,"parent_record_offset":14972,"ancestor_record_offset":13596,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":230,"record_offset":14208,"parent_record_offset":14180,"ancestor_record_offset":13596,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":230,"record_offset":14180,"parent_record_offset":14156,"ancestor_record_offset":13596,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":230,"record_offset":15060,"parent_record_offset":15028,"ancestor_record_offset":13596,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":230,"record_offset":15028,"parent_record_offset":15000,"ancestor_record_offset":13596,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":230,"record_offset":14156,"parent_record_offset":14132,"ancestor_record_offset":13596,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":230,"record_offset":16940,"parent_record_offset":16900,"ancestor_record_offset":13596,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":230,"record_offset":14880,"parent_record_offset":14852,"ancestor_record_offset":13596,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":230,"record_offset":14908,"parent_record_offset":14880,"ancestor_record_offset":13596,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":230,"record_offset":14936,"parent_record_offset":14908,"ancestor_record_offset":13596,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":230,"record_offset":14852,"parent_record_offset":14824,"ancestor_record_offset":13596,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":230,"record_offset":14824,"parent_record_offset":14796,"ancestor_record_offset":13596,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":230,"record_offset":14796,"parent_record_offset":14768,"ancestor_record_offset":13596,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":230,"record_offset":18124,"parent_record_offset":18088,"ancestor_record_offset":13596,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":230,"record_offset":14424,"parent_record_offset":14400,"ancestor_record_offset":13596,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":230,"record_offset":16748,"parent_record_offset":16712,"ancestor_record_offset":13596,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":230,"record_offset":14972,"parent_record_offset":14936,"ancestor_record_offset":13596,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":230,"record_offset":16684,"parent_record_offset":16656,"ancestor_record_offset":13596,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":230,"record_offset":15980,"parent_record_offset":15948,"ancestor_record_offset":13596,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":230,"record_offset":16712,"parent_record_offset":16684,"ancestor_record_offset":13596,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] alloc::wtf8::impl$14::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\wtf8\mod.rs
//     evidence={"module":230,"record_offset":16656,"parent_record_offset":16628,"ancestor_record_offset":13596,"inlinee":6712,"source_line":38,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":230,"record_offset":16964,"parent_record_offset":16940,"ancestor_record_offset":13596,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":230,"record_offset":17012,"parent_record_offset":16988,"ancestor_record_offset":13596,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":230,"record_offset":16988,"parent_record_offset":16964,"ancestor_record_offset":13596,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":230,"record_offset":16148,"parent_record_offset":16120,"ancestor_record_offset":13596,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":230,"record_offset":17968,"parent_record_offset":17944,"ancestor_record_offset":13596,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":230,"record_offset":15216,"parent_record_offset":14936,"ancestor_record_offset":13596,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":230,"record_offset":15240,"parent_record_offset":15216,"ancestor_record_offset":13596,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=38
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":230,"record_offset":14060,"parent_record_offset":14036,"ancestor_record_offset":13596,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":230,"record_offset":18156,"parent_record_offset":18124,"ancestor_record_offset":13596,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":230,"record_offset":14460,"parent_record_offset":14424,"ancestor_record_offset":13596,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":230,"record_offset":14484,"parent_record_offset":14460,"ancestor_record_offset":13596,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":230,"record_offset":14516,"parent_record_offset":14424,"ancestor_record_offset":13596,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] sentry_types::protocol::v7::impl$2::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"sentry-types","version":"0.31.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sentry-types-0.31.7/src/protocol/v7.rs
//     evidence={"module":230,"record_offset":15884,"parent_record_offset":15860,"ancestor_record_offset":13596,"inlinee":10154,"source_line":84,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":230,"record_offset":16600,"parent_record_offset":16572,"ancestor_record_offset":13596,"inlinee":6708,"source_line":699,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$7::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":230,"record_offset":16628,"parent_record_offset":16600,"ancestor_record_offset":13596,"inlinee":6710,"source_line":70,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$6::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/panic_report.rs
//     evidence={"module":230,"record_offset":17920,"parent_record_offset":17888,"ancestor_record_offset":13596,"inlinee":17129,"source_line":198,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$6::closure$0
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 159
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x42F2
// 首条位置: {"module":230,"inlinee":17138}
// 编译实例 RVAs: 0x5e64a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t783377","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374895264,"raw_hex":"e811f40b00","size":5,"soft_fail":false,"target_va":5375678704,"va":5374895322,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: panic_report.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42161695","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374895264,"raw_hex":"e81f568302","size":5,"soft_fail":false,"target_va":5417057056,"va":5374895356,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":230,"record_offset":412,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":230,"record_offset":612,"parent_record_offset":588,"ancestor_record_offset":248,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":230,"record_offset":636,"parent_record_offset":612,"ancestor_record_offset":248,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":230,"record_offset":660,"parent_record_offset":636,"ancestor_record_offset":248,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":230,"record_offset":588,"parent_record_offset":536,"ancestor_record_offset":248,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":230,"record_offset":560,"parent_record_offset":536,"ancestor_record_offset":248,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":230,"record_offset":536,"parent_record_offset":512,"ancestor_record_offset":248,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":230,"record_offset":512,"parent_record_offset":492,"ancestor_record_offset":248,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":230,"record_offset":432,"parent_record_offset":412,"ancestor_record_offset":248,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$6::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 198
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x42E9
// 首条位置: {"module":230,"inlinee":17129}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":230,"record_offset":17944,"parent_record_offset":17920,"ancestor_record_offset":17920,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":230,"record_offset":17992,"parent_record_offset":17968,"ancestor_record_offset":17920,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":230,"record_offset":17968,"parent_record_offset":17944,"ancestor_record_offset":17920,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$7
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 221
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x409D
// 首条位置: {"module":211,"inlinee":16541}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":211,"record_offset":27328,"parent_record_offset":27300,"ancestor_record_offset":27300,"inlinee":4498,"source_line":1884,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":211,"record_offset":27396,"parent_record_offset":27300,"ancestor_record_offset":27300,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_direct] std::thread::functions::spawn
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\functions.rs
//     evidence={"module":211,"record_offset":27432,"parent_record_offset":27300,"ancestor_record_offset":27300,"inlinee":7489,"source_line":125,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":211,"record_offset":27356,"parent_record_offset":27328,"ancestor_record_offset":27300,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":211,"record_offset":27552,"parent_record_offset":27432,"ancestor_record_offset":27300,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":211,"record_offset":27512,"parent_record_offset":27488,"ancestor_record_offset":27300,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::builder::Builder::spawn
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\builder.rs
//     evidence={"module":211,"record_offset":27464,"parent_record_offset":27432,"ancestor_record_offset":27300,"inlinee":7491,"source_line":185,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::builder::Builder::spawn_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\builder.rs
//     evidence={"module":211,"record_offset":27488,"parent_record_offset":27464,"ancestor_record_offset":27300,"inlinee":7492,"source_line":254,"archive":"_pdb/C/modules/00211.jsonl.gz"}
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
// 完整结构及原始类型记录: panic_report.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: panic_report.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: panic_report.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 8; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)()
// 候选参数类型: []
// [文件级接口候选，具体函数归属 unknown] country_core::render::render_config::VideoMode (*)(ref$<winit::monitor::VideoMode>)
// 候选参数类型: ["ref$<winit::monitor::VideoMode>"]
// [文件级接口候选，具体函数归属 unknown] country_core::render::render_config::LastKnownGraphicsDevice (*)(ref$<rhapsody::raw::init::AvailableGraphicsDevice>)
// 候选参数类型: ["ref$<rhapsody::raw::init::AvailableGraphicsDevice>"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(ref$<glam::f32::vec2::Vec2>)
// 候选参数类型: ["ref$<glam::f32::vec2::Vec2>"]
// [文件级接口候选，具体函数归属 unknown] iced_core::rectangle::Rectangle<f32> (*)(ref$<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >)
// 候选参数类型: ["ref$<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >"]
// [文件级接口候选，具体函数归属 unknown] std::ffi::os_str::OsString (*)(ref$<alloc::string::String>)
// 候选参数类型: ["ref$<alloc::string::String>"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] tuple$<bool,tuple$<> > (*)(ref_mut$<bevy_ecs::storage::sparse_set::SparseSets>,enum2$<core::option::Option<ref_mut$<bevy_ecs::storage::table::Table> > >,ref$<bevy_ecs::component::info::Components>,ref$<slice2$<bevy_ecs::component::info::ComponentId> >)
// 候选参数类型: ["ref_mut$<bevy_ecs::storage::sparse_set::SparseSets>", "enum2$<core::option::Option<ref_mut$<bevy_ecs::storage::table::Table> > >", "ref$<bevy_ecs::component::info::Components>", "ref$<slice2$<bevy_ecs::component::info::ComponentId> >"]

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_report::on_game_crashed
// 信息 ID: a6546272bf4fe39cccf970f6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::main @ 0x6423fd -> 0x541c50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$0
// 信息 ID: ca26b28dc0bf0a19337be391
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::panic_reporter::panic_report::on_game_crashed", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::panic_report::on_game_crashed; module=189, inline_record=2200; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$1
// 信息 ID: 5646c4964ed0ffbc38224ee1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::panic_reporter::panic_report::on_game_crashed", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::panic_report::on_game_crashed; module=189, inline_record=2300; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$2
// 信息 ID: 7b924679139ddeddbdb26f2a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "tiny_glade::panic_reporter::panic_report::on_game_crashed", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::panic_report::on_game_crashed; module=189, inline_record=2372; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$2::closure$0
// 信息 ID: ca0187ac6c3f7acf148cf231
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "tiny_glade::panic_reporter::panic_report::on_game_crashed", "captures": "unknown"}
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::panic_reporter::panic_report::on_game_crashed @ 0x5422ee -> 0x2d71a70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$4
// 信息 ID: f1c0f0ee950fcbc7260b3091
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "tiny_glade::panic_reporter::panic_report::on_game_crashed", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::panic_report::on_game_crashed; module=189, inline_record=18420; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$5
// 信息 ID: 44fd14b884f89df5f5735ab4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 5, "lexical_owner": "tiny_glade::panic_reporter::panic_report::on_game_crashed", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::panic_report::on_game_crashed; module=189, inline_record=18588; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$5::closure$0
// 信息 ID: dd0542f7a3ba621cb8273a6f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 5, "lexical_owner": "tiny_glade::panic_reporter::panic_report::on_game_crashed", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$5::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<alloc::string::String,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<sentry_types::protocol::v7::Frame>,tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$5::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<alloc::string::String,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<sentry_types::protocol::v7::Frame>,tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$5::closure_env$0> >; module=81, inline_record=149708; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<alloc::string::String,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<sentry_types::protocol::v7::Frame>,tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$5::closure_env$0> >; module=81, inline_record=150412; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$6
// 信息 ID: 0748898dd80a3bc56874fa77
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 6, "lexical_owner": "tiny_glade::panic_reporter::panic_report::on_game_crashed", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$6 <- std::thread::lifecycle::spawn_unchecked<tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$6,enum2$<core::result::Result<alloc::string::String,anyhow::Error> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$6 <- core::ops::function::FnOnce::call_once<std::thread::lifecycle::spawn_unchecked::closure_env$1<tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$6,enum2$<core::result::Result<alloc::string::String,anyhow::Error> > >,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$6 <- core::ptr::drop_in_place<tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$6>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$6 <- core::ptr::drop_in_place<std::thread::lifecycle::spawn_unchecked::closure_env$1<tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$6,enum2$<core::result::Result<alloc::string::String,anyhow::Error> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$6 <- core::ptr::drop_in_place<enum2$<core::option::Option<tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$6> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$6 <- std::sys::backtrace::__rust_begin_short_backtrace<tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$6,enum2$<core::result::Result<alloc::string::String,anyhow::Error> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 6 条。
// [PDB 内联宿主] std::sys::backtrace::__rust_begin_short_backtrace<tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$6,enum2$<core::result::Result<alloc::string::String,anyhow::Error> > >; module=230, inline_record=13596; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$6::closure$0
// 信息 ID: 6f15bdb48f312eca0419d9e4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 6, "lexical_owner": "tiny_glade::panic_reporter::panic_report::on_game_crashed", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$6::closure_env$0 <- core::ops::function::FnOnce::call_once<tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$6::closure_env$0,tuple$<sentry_types::protocol::v7::Event> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$6::closure_env$0,tuple$<sentry_types::protocol::v7::Event> >; module=230, inline_record=25228; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$6::closure$1
// 信息 ID: e792ec8b81a5527e14648d27
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 6, "lexical_owner": "tiny_glade::panic_reporter::panic_report::on_game_crashed", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] std::sys::backtrace::__rust_begin_short_backtrace<tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$6,enum2$<core::result::Result<alloc::string::String,anyhow::Error> > >; module=230, inline_record=17920; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$7
// 信息 ID: 28864f74f6b95cc613470e6f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 7, "lexical_owner": "tiny_glade::panic_reporter::panic_report::on_game_crashed", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$7 <- crash_reporter_gui::run_crash_reporter_gui<tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$7>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$7 <- core::ops::function::FnOnce::call_once<tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$7,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$7 <- core::ptr::drop_in_place<tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$7>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$7,tuple$<> >; module=211, inline_record=27300; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
