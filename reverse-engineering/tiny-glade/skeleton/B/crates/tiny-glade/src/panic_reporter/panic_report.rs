// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/tiny-glade/src/panic_reporter/panic_report.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: c9cf07ff1e76a373d63712c5c0698afff7b97bffdeecfe9530fd29c36631c95c
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: panic_report.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 31, 104, 131, 137, 157
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.86"}]
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
// 源码行: 33
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":157,"record_offset":38484,"rva":3860800,"va":5372569920}
// 编译实例 RVAs: 0x3ae940
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$alloc..string..String$u20$as$u20$core..clone..Clone$GT$::clone
//     owner=unresolved_namespace; count=12
//     evidence={"assembly":"\tcallq\t10277600","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372569920,"raw_hex":"e8e0d29c00","size":5,"soft_fail":false,"target_va":5382847888,"va":5372570283,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=7
//     evidence={"assembly":"\tcallq\t10074309","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372569920,"raw_hex":"e8c5b89900","size":5,"soft_fail":false,"target_va":5382644528,"va":5372570214,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$tempfile..dir..TempDir$u20$as$u20$core..ops..drop..Drop$GT$::drop
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8203193","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372569920,"raw_hex":"e8b92b7d00","size":5,"soft_fail":false,"target_va":5380782192,"va":5372578994,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14002e900
//     owner=unresolved_namespace; count=1
//     target_aliases=130 candidates; complete list: panic_report.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3681375","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372569920,"raw_hex":"e8a1d3c7ff","size":5,"soft_fail":false,"target_va":5368899840,"va":5372581210,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400d9b90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: panic_report.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2982331","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372569920,"raw_hex":"e8457ed2ff","size":5,"soft_fail":false,"target_va":5369600912,"va":5372583238,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14014b4d0
//     owner=unresolved_namespace; count=1
//     target_aliases=14 candidates; complete list: panic_report.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2515752","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372569920,"raw_hex":"e8d89cd9ff","size":5,"soft_fail":false,"target_va":5370066128,"va":5372581875,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14014d960
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: panic_report.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2503354","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372569920,"raw_hex":"e846cdd9ff","size":5,"soft_fail":false,"target_va":5370075488,"va":5372578837,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402b7a20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: panic_report.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1013553","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372569920,"raw_hex":"e8cf88f0ff","size":5,"soft_fail":false,"target_va":5371558432,"va":5372571980,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402be160
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: panic_report.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-992323","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372569920,"raw_hex":"e8bddbf0ff","size":5,"soft_fail":false,"target_va":5371584864,"va":5372577182,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140338bb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: panic_report.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-490817","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372569920,"raw_hex":"e8bf82f8ff","size":5,"soft_fail":false,"target_va":5372087216,"va":5372578028,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14033a860
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: panic_report.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-488129","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372569920,"raw_hex":"e83f8df8ff","size":5,"soft_fail":false,"target_va":5372094560,"va":5372582684,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404b1430
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: panic_report.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1050136","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372569920,"raw_hex":"e818061000","size":5,"soft_fail":false,"target_va":5373629488,"va":5372579347,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d0570
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: panic_report.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2221709","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372569920,"raw_hex":"e88de62100","size":5,"soft_fail":false,"target_va":5374805360,"va":5372583646,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e23a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: panic_report.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2296465","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372569920,"raw_hex":"e8910a2300","size":5,"soft_fail":false,"target_va":5374878624,"va":5372582154,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7a00
//     owner=unresolved_namespace; count=2
//     target_aliases=92 candidates; complete list: panic_report.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2319960","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372569920,"raw_hex":"e858662300","size":5,"soft_fail":false,"target_va":5374900736,"va":5372580771,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e9a40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: panic_report.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2335859","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372569920,"raw_hex":"e873a42300","size":5,"soft_fail":false,"target_va":5374908992,"va":5372573128,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405f8f70
//     owner=unresolved_namespace; count=2
//     target_aliases=18 candidates; complete list: panic_report.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2390362","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372569920,"raw_hex":"e85a792400","size":5,"soft_fail":false,"target_va":5374971760,"va":5372581393,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405fff90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: panic_report.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2416462","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372569920,"raw_hex":"e84edf2400","size":5,"soft_fail":false,"target_va":5375000464,"va":5372583997,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406009e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: panic_report.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2419605","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372569920,"raw_hex":"e895eb2400","size":5,"soft_fail":false,"target_va":5375003104,"va":5372583494,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406023d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: panic_report.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2430541","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372569920,"raw_hex":"e84d162500","size":5,"soft_fail":false,"target_va":5375009744,"va":5372579198,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=15
//     target_aliases=1 candidates; complete list: panic_report.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2511029","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372569920,"raw_hex":"e8b5502600","size":5,"soft_fail":false,"target_va":5375081488,"va":5372570454,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=45
//     target_aliases=1 candidates; complete list: panic_report.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2511186","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372569920,"raw_hex":"e852512600","size":5,"soft_fail":false,"target_va":5375081504,"va":5372570313,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=8
//     target_aliases=69 candidates; complete list: panic_report.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2945990","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372569920,"raw_hex":"e8c6f32c00","size":5,"soft_fail":false,"target_va":5375516032,"va":5372570037,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b83810
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: panic_report.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8211752","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372569920,"raw_hex":"e8284d7d00","size":5,"soft_fail":false,"target_va":5380782096,"va":5372570339,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d61b00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: panic_report.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10170221","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372569920,"raw_hex":"e86d2f9b00","size":5,"soft_fail":false,"target_va":5382740736,"va":5372570510,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76ed0
//     owner=unresolved_namespace; count=11
//     target_aliases=5 candidates; complete list: panic_report.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10256841","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372569920,"raw_hex":"e8c9819c00","size":5,"soft_fail":false,"target_va":5382827728,"va":5372570882,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a2020
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: panic_report.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38744939","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372569920,"raw_hex":"e86b334f02","size":5,"soft_fail":false,"target_va":5411315744,"va":5372570800,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=18
//     target_aliases=1 candidates; complete list: panic_report.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39276628","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372569920,"raw_hex":"e854505702","size":5,"soft_fail":false,"target_va":5411847168,"va":5372570535,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38961580","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372569920,"raw_hex":"e8ac815202","size":5,"soft_fail":false,"target_va":5411540992,"va":5372579407,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10264099","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372569920,"raw_hex":"e8239e9c00","size":5,"soft_fail":false,"target_va":5382840208,"va":5372576104,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=14
//     evidence={"assembly":"\tcallq\t38961487","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372569920,"raw_hex":"e84f815202","size":5,"soft_fail":false,"target_va":5411540963,"va":5372579471,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10083085","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372569920,"raw_hex":"e80ddb9900","size":5,"soft_fail":false,"target_va":5382655024,"va":5372571934,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38962148","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372569920,"raw_hex":"e8e4835202","size":5,"soft_fail":false,"target_va":5411541584,"va":5372579431,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<enum2$<core::result::Result<alloc::string::String,anyhow::Error> >,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-22523","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372569920,"raw_hex":"e805a8ffff","size":5,"soft_fail":false,"target_va":5372559440,"va":5372581958,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<std::path::PathBuf,alloc::string::String> > >
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t-3501480","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372569920,"raw_hex":"e85892caff","size":5,"soft_fail":false,"target_va":5369080752,"va":5372582227,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<std::path::PathBuf,anyhow::Error> > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-24095","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372569920,"raw_hex":"e8e1a1ffff","size":5,"soft_fail":false,"target_va":5372559664,"va":5372583754,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<tempfile::dir::TempDir,std::io::error::Error> > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-24072","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372569920,"raw_hex":"e8f8a1ffff","size":5,"soft_fail":false,"target_va":5372559808,"va":5372583875,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<tiny_glade::crash_pickup::crash_payload::RecoveredSave,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError> > > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-3504104","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372569920,"raw_hex":"e81888caff","size":5,"soft_fail":false,"target_va":5369078928,"va":5372583027,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,enum2$<tg_package_integrity::PackageVerificationError> > > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-24092","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372569920,"raw_hex":"e8e4a1ffff","size":5,"soft_fail":false,"target_va":5372559024,"va":5372583111,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::load_last_used_locale
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8315330","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372569920,"raw_hex":"e8c2e17e00","size":5,"soft_fail":false,"target_va":5380886912,"va":5372571577,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirBuilder::_create
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10133733","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372569920,"raw_hex":"e8e5a09a00","size":5,"soft_fail":false,"target_va":5382709584,"va":5372575846,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t10171360","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372569920,"raw_hex":"e8e0339b00","size":5,"soft_fail":false,"target_va":5382744080,"va":5372572715,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::pal::windows::fs::canonicalize
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10207770","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372569920,"raw_hex":"e81ac29b00","size":5,"soft_fail":false,"target_va":5382778672,"va":5372570897,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::pal::windows::fs::copy
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t10206052","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372569920,"raw_hex":"e864bb9b00","size":5,"soft_fail":false,"target_va":5382778992,"va":5372572935,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::pal::windows::fs::rename
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10200081","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372569920,"raw_hex":"e811a49b00","size":5,"soft_fail":false,"target_va":5382772848,"va":5372572762,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::JoinHandle$LT$T$GT$::join
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-1214454","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372569920,"raw_hex":"e80a78edff","size":5,"soft_fail":false,"target_va":5371363664,"va":5372578113,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_package_integrity::verify_package_manifest
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-package-integrity","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2932668","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372569920,"raw_hex":"e8bcbf2c00","size":5,"soft_fail":false,"target_va":5375509456,"va":5372576783,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1060855","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372569920,"raw_hex":"e8f72f1000","size":5,"soft_fail":false,"target_va":5373636528,"va":5372575668,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::file::date_time_string_for_filename
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9332118","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372569920,"raw_hex":"e896658e00","size":5,"soft_fail":false,"target_va":5381908112,"va":5372575989,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=7
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5372569920,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5372571094,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":157,"record_offset":58448,"parent_record_offset":38484,"ancestor_record_offset":38484,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] alloc::rc::Rc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\rc.rs
//     evidence={"module":157,"record_offset":62868,"parent_record_offset":38484,"ancestor_record_offset":38484,"inlinee":9428,"source_line":410,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] alloc::rc::impl$25::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\rc.rs
//     evidence={"module":157,"record_offset":63392,"parent_record_offset":38484,"ancestor_record_offset":38484,"inlinee":6790,"source_line":2248,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":157,"record_offset":62548,"parent_record_offset":38484,"ancestor_record_offset":38484,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] core::cell::RefCell::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":157,"record_offset":62840,"parent_record_offset":38484,"ancestor_record_offset":38484,"inlinee":8953,"source_line":850,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] core::cell::RefCell::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":157,"record_offset":63476,"parent_record_offset":38484,"ancestor_record_offset":38484,"inlinee":8232,"source_line":1288,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":157,"record_offset":38876,"parent_record_offset":38484,"ancestor_record_offset":38484,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":157,"record_offset":66880,"parent_record_offset":38484,"ancestor_record_offset":38484,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hint.rs
//     evidence={"module":157,"record_offset":58532,"parent_record_offset":38484,"ancestor_record_offset":38484,"inlinee":7064,"source_line":599,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":157,"record_offset":62576,"parent_record_offset":38484,"ancestor_record_offset":38484,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":157,"record_offset":38736,"parent_record_offset":38484,"ancestor_record_offset":38484,"inlinee":5657,"source_line":651,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":157,"record_offset":62604,"parent_record_offset":38484,"ancestor_record_offset":38484,"inlinee":5577,"source_line":1173,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":157,"record_offset":39880,"parent_record_offset":38484,"ancestor_record_offset":38484,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":157,"record_offset":40348,"parent_record_offset":38484,"ancestor_record_offset":38484,"inlinee":4667,"source_line":1351,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":157,"record_offset":40136,"parent_record_offset":38484,"ancestor_record_offset":38484,"inlinee":8221,"source_line":709,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::is_ok
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":157,"record_offset":54596,"parent_record_offset":38484,"ancestor_record_offset":38484,"inlinee":6884,"source_line":564,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":157,"record_offset":40164,"parent_record_offset":38484,"ancestor_record_offset":38484,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":157,"record_offset":40256,"parent_record_offset":38484,"ancestor_record_offset":38484,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":157,"record_offset":62216,"parent_record_offset":38484,"ancestor_record_offset":38484,"inlinee":4624,"source_line":658,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$5::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":157,"record_offset":39084,"parent_record_offset":38484,"ancestor_record_offset":38484,"inlinee":13528,"source_line":1730,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":157,"record_offset":38908,"parent_record_offset":38484,"ancestor_record_offset":38484,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":157,"record_offset":38792,"parent_record_offset":38484,"ancestor_record_offset":38484,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] std::fs::copy
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":157,"record_offset":55456,"parent_record_offset":38484,"ancestor_record_offset":38484,"inlinee":12455,"source_line":2523,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] std::fs::create_dir_all
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":157,"record_offset":57200,"parent_record_offset":38484,"ancestor_record_offset":38484,"inlinee":8207,"source_line":2766,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] std::fs::rename
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":157,"record_offset":53952,"parent_record_offset":38484,"ancestor_record_offset":38484,"inlinee":12457,"source_line":2462,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":157,"record_offset":53924,"parent_record_offset":38484,"ancestor_record_offset":38484,"inlinee":5199,"source_line":2688,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$45::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":157,"record_offset":53784,"parent_record_offset":38484,"ancestor_record_offset":38484,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::crash_pickup::crash_payload::impl$7::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":157,"record_offset":42860,"parent_record_offset":38484,"ancestor_record_offset":38484,"inlinee":13537,"source_line":26,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":157,"record_offset":39712,"parent_record_offset":39688,"ancestor_record_offset":38484,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":157,"record_offset":39736,"parent_record_offset":39712,"ancestor_record_offset":38484,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=46
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":157,"record_offset":40080,"parent_record_offset":40056,"ancestor_record_offset":38484,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":157,"record_offset":62940,"parent_record_offset":62904,"ancestor_record_offset":38484,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":157,"record_offset":39688,"parent_record_offset":39512,"ancestor_record_offset":38484,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=46
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":157,"record_offset":40056,"parent_record_offset":40000,"ancestor_record_offset":38484,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":157,"record_offset":62904,"parent_record_offset":62868,"ancestor_record_offset":38484,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":157,"record_offset":41056,"parent_record_offset":41028,"ancestor_record_offset":38484,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":157,"record_offset":58496,"parent_record_offset":58472,"ancestor_record_offset":38484,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":39452,"parent_record_offset":39424,"ancestor_record_offset":38484,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":40028,"parent_record_offset":40000,"ancestor_record_offset":38484,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":40000,"parent_record_offset":39976,"ancestor_record_offset":38484,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":39512,"parent_record_offset":39480,"ancestor_record_offset":38484,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":39480,"parent_record_offset":39452,"ancestor_record_offset":38484,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":39976,"parent_record_offset":39952,"ancestor_record_offset":38484,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":39548,"parent_record_offset":39512,"ancestor_record_offset":38484,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::rc::Rc::inner
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\rc.rs
//     evidence={"module":157,"record_offset":63416,"parent_record_offset":63392,"ancestor_record_offset":38484,"inlinee":6753,"source_line":360,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::rc::impl$32::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\rc.rs
//     evidence={"module":157,"record_offset":65128,"parent_record_offset":65104,"ancestor_record_offset":38484,"inlinee":6749,"source_line":2301,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":157,"record_offset":46024,"parent_record_offset":45984,"ancestor_record_offset":38484,"inlinee":11743,"source_line":124,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":157,"record_offset":39388,"parent_record_offset":39356,"ancestor_record_offset":38484,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":157,"record_offset":39356,"parent_record_offset":39288,"ancestor_record_offset":38484,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":157,"record_offset":39328,"parent_record_offset":39288,"ancestor_record_offset":38484,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":157,"record_offset":39424,"parent_record_offset":39388,"ancestor_record_offset":38484,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$11::clone
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":157,"record_offset":39256,"parent_record_offset":39224,"ancestor_record_offset":38484,"inlinee":4623,"source_line":3286,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":157,"record_offset":62772,"parent_record_offset":62748,"ancestor_record_offset":38484,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":157,"record_offset":39288,"parent_record_offset":39256,"ancestor_record_offset":38484,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":157,"record_offset":41088,"parent_record_offset":41056,"ancestor_record_offset":38484,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":157,"record_offset":39572,"parent_record_offset":39548,"ancestor_record_offset":38484,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":157,"record_offset":39596,"parent_record_offset":39572,"ancestor_record_offset":38484,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":157,"record_offset":63612,"parent_record_offset":63588,"ancestor_record_offset":38484,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":157,"record_offset":63772,"parent_record_offset":63748,"ancestor_record_offset":38484,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":157,"record_offset":63748,"parent_record_offset":63724,"ancestor_record_offset":38484,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":157,"record_offset":63544,"parent_record_offset":63508,"ancestor_record_offset":38484,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":157,"record_offset":63508,"parent_record_offset":63476,"ancestor_record_offset":38484,"inlinee":8196,"source_line":902,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":157,"record_offset":63588,"parent_record_offset":63544,"ancestor_record_offset":38484,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":157,"record_offset":63724,"parent_record_offset":63700,"ancestor_record_offset":38484,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":157,"record_offset":53976,"parent_record_offset":53952,"ancestor_record_offset":38484,"inlinee":5126,"source_line":703,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":157,"record_offset":40312,"parent_record_offset":40288,"ancestor_record_offset":38484,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":157,"record_offset":39812,"parent_record_offset":39788,"ancestor_record_offset":38484,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":157,"record_offset":39644,"parent_record_offset":39620,"ancestor_record_offset":38484,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":157,"record_offset":46524,"parent_record_offset":46480,"ancestor_record_offset":38484,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::take::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\take.rs
//     evidence={"module":157,"record_offset":46480,"parent_record_offset":46024,"ancestor_record_offset":38484,"inlinee":11744,"source_line":36,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":157,"record_offset":62748,"parent_record_offset":62640,"ancestor_record_offset":38484,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":157,"record_offset":46624,"parent_record_offset":46024,"ancestor_record_offset":38484,"inlinee":7961,"source_line":510,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":157,"record_offset":63796,"parent_record_offset":63772,"ancestor_record_offset":38484,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":157,"record_offset":41112,"parent_record_offset":41088,"ancestor_record_offset":38484,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":157,"record_offset":39620,"parent_record_offset":39596,"ancestor_record_offset":38484,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":157,"record_offset":38760,"parent_record_offset":38736,"ancestor_record_offset":38484,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":157,"record_offset":58472,"parent_record_offset":58448,"ancestor_record_offset":38484,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":157,"record_offset":45648,"parent_record_offset":44772,"ancestor_record_offset":38484,"inlinee":4547,"source_line":2037,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":157,"record_offset":38964,"parent_record_offset":38932,"ancestor_record_offset":38484,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":157,"record_offset":38992,"parent_record_offset":38932,"ancestor_record_offset":38484,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":157,"record_offset":39788,"parent_record_offset":39388,"ancestor_record_offset":38484,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=122
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":157,"record_offset":39904,"parent_record_offset":39880,"ancestor_record_offset":38484,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":157,"record_offset":62712,"parent_record_offset":62688,"ancestor_record_offset":38484,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":157,"record_offset":40856,"parent_record_offset":40828,"ancestor_record_offset":38484,"inlinee":5966,"source_line":1340,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":157,"record_offset":40904,"parent_record_offset":40880,"ancestor_record_offset":38484,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":157,"record_offset":40880,"parent_record_offset":40856,"ancestor_record_offset":38484,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":157,"record_offset":63440,"parent_record_offset":63416,"ancestor_record_offset":38484,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":157,"record_offset":46584,"parent_record_offset":46552,"ancestor_record_offset":38484,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":157,"record_offset":63644,"parent_record_offset":63544,"ancestor_record_offset":38484,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":157,"record_offset":63048,"parent_record_offset":63020,"ancestor_record_offset":38484,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":157,"record_offset":63820,"parent_record_offset":63796,"ancestor_record_offset":38484,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":157,"record_offset":40640,"parent_record_offset":40396,"ancestor_record_offset":38484,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$5::clone
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":157,"record_offset":43080,"parent_record_offset":42860,"ancestor_record_offset":38484,"inlinee":13528,"source_line":1730,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":157,"record_offset":62664,"parent_record_offset":62640,"ancestor_record_offset":38484,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":157,"record_offset":62688,"parent_record_offset":62664,"ancestor_record_offset":38484,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":157,"record_offset":46552,"parent_record_offset":46524,"ancestor_record_offset":38484,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":157,"record_offset":38816,"parent_record_offset":38792,"ancestor_record_offset":38484,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":157,"record_offset":38840,"parent_record_offset":38816,"ancestor_record_offset":38484,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::grading_lut::impl$5::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/grading_lut.rs
//     evidence={"module":157,"record_offset":46740,"parent_record_offset":46708,"ancestor_record_offset":38484,"inlinee":13524,"source_line":24,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::image_formation::impl$12::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/image_formation.rs
//     evidence={"module":157,"record_offset":46708,"parent_record_offset":46672,"ancestor_record_offset":38484,"inlinee":13522,"source_line":27,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::image_formation::impl$2::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/image_formation.rs
//     evidence={"module":157,"record_offset":46784,"parent_record_offset":46708,"ancestor_record_offset":38484,"inlinee":13526,"source_line":11,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::impl$28::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":157,"record_offset":45808,"parent_record_offset":45768,"ancestor_record_offset":38484,"inlinee":13520,"source_line":181,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::impl$29::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":157,"record_offset":45768,"parent_record_offset":45696,"ancestor_record_offset":38484,"inlinee":13518,"source_line":256,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":157,"record_offset":39020,"parent_record_offset":38932,"ancestor_record_offset":38484,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":157,"record_offset":38932,"parent_record_offset":38908,"ancestor_record_offset":38484,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":157,"record_offset":39044,"parent_record_offset":39020,"ancestor_record_offset":38484,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":157,"record_offset":39160,"parent_record_offset":39128,"ancestor_record_offset":38484,"inlinee":8126,"source_line":642,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":157,"record_offset":40484,"parent_record_offset":40460,"ancestor_record_offset":38484,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":157,"record_offset":40532,"parent_record_offset":40508,"ancestor_record_offset":38484,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":157,"record_offset":40508,"parent_record_offset":40484,"ancestor_record_offset":38484,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::create
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":157,"record_offset":57448,"parent_record_offset":57200,"ancestor_record_offset":38484,"inlinee":8210,"source_line":3032,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::recursive
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":157,"record_offset":57224,"parent_record_offset":57200,"ancestor_record_offset":38484,"inlinee":8209,"source_line":3008,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::canonicalize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":157,"record_offset":40604,"parent_record_offset":40576,"ancestor_record_offset":38484,"inlinee":9323,"source_line":2678,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":157,"record_offset":40828,"parent_record_offset":40800,"ancestor_record_offset":38484,"inlinee":5965,"source_line":253,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":157,"record_offset":40800,"parent_record_offset":40772,"ancestor_record_offset":38484,"inlinee":5963,"source_line":239,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::canonicalize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":157,"record_offset":40576,"parent_record_offset":40396,"ancestor_record_offset":38484,"inlinee":9322,"source_line":2951,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":157,"record_offset":40460,"parent_record_offset":40436,"ancestor_record_offset":38484,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$30::clone
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":157,"record_offset":39128,"parent_record_offset":39084,"ancestor_record_offset":38484,"inlinee":8124,"source_line":1738,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":157,"record_offset":40436,"parent_record_offset":40396,"ancestor_record_offset":38484,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$59::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":157,"record_offset":40220,"parent_record_offset":40196,"ancestor_record_offset":38484,"inlinee":13531,"source_line":2045,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$83::as_ref
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":157,"record_offset":54000,"parent_record_offset":53976,"ancestor_record_offset":38484,"inlinee":5128,"source_line":3362,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$10::clone
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":157,"record_offset":39192,"parent_record_offset":39160,"ancestor_record_offset":38484,"inlinee":8128,"source_line":13,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::sys_common::wtf8::impl$35::clone
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys_common\wtf8.rs
//     evidence={"module":157,"record_offset":39224,"parent_record_offset":39192,"ancestor_record_offset":38484,"inlinee":8130,"source_line":132,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::impl$6::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":157,"record_offset":44736,"parent_record_offset":44664,"ancestor_record_offset":38484,"inlinee":13539,"source_line":17,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::impl$8::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":157,"record_offset":44772,"parent_record_offset":44664,"ancestor_record_offset":38484,"inlinee":13541,"source_line":40,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::impl$9::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":157,"record_offset":45696,"parent_record_offset":45648,"ancestor_record_offset":38484,"inlinee":13543,"source_line":46,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/panic_report.rs
//     evidence={"module":157,"record_offset":40196,"parent_record_offset":40164,"ancestor_record_offset":38484,"inlinee":13529,"source_line":48,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/panic_report.rs
//     evidence={"module":157,"record_offset":40288,"parent_record_offset":40256,"ancestor_record_offset":38484,"inlinee":13532,"source_line":52,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/panic_report.rs
//     evidence={"module":157,"record_offset":40396,"parent_record_offset":40348,"ancestor_record_offset":38484,"inlinee":13533,"source_line":53,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$2::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/panic_report.rs
//     evidence={"module":157,"record_offset":40684,"parent_record_offset":40640,"ancestor_record_offset":38484,"inlinee":13535,"source_line":53,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$4
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/panic_report.rs
//     evidence={"module":157,"record_offset":62640,"parent_record_offset":62604,"ancestor_record_offset":38484,"inlinee":13544,"source_line":151,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 48
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x34D9
// 首条位置: {"module":157,"inlinee":13529}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::path::impl$59::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":157,"record_offset":40220,"parent_record_offset":40196,"ancestor_record_offset":40196,"inlinee":13531,"source_line":2045,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 52
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x34DC
// 首条位置: {"module":157,"inlinee":13532}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":157,"record_offset":40312,"parent_record_offset":40288,"ancestor_record_offset":40288,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 53
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x34DD
// 首条位置: {"module":157,"inlinee":13533}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":157,"record_offset":41552,"parent_record_offset":40396,"ancestor_record_offset":40396,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":157,"record_offset":40640,"parent_record_offset":40396,"ancestor_record_offset":40396,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::canonicalize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":157,"record_offset":40576,"parent_record_offset":40396,"ancestor_record_offset":40396,"inlinee":9322,"source_line":2951,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$45::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":157,"record_offset":40436,"parent_record_offset":40396,"ancestor_record_offset":40396,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":157,"record_offset":41168,"parent_record_offset":41144,"ancestor_record_offset":40396,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":157,"record_offset":41144,"parent_record_offset":41056,"ancestor_record_offset":40396,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":157,"record_offset":41056,"parent_record_offset":41028,"ancestor_record_offset":40396,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":41780,"parent_record_offset":41748,"ancestor_record_offset":40396,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":41748,"parent_record_offset":41720,"ancestor_record_offset":40396,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":41720,"parent_record_offset":41692,"ancestor_record_offset":40396,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":157,"record_offset":41088,"parent_record_offset":41056,"ancestor_record_offset":40396,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":157,"record_offset":40716,"parent_record_offset":40684,"ancestor_record_offset":40396,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":157,"record_offset":41112,"parent_record_offset":41088,"ancestor_record_offset":40396,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":157,"record_offset":40744,"parent_record_offset":40684,"ancestor_record_offset":40396,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":157,"record_offset":40856,"parent_record_offset":40828,"ancestor_record_offset":40396,"inlinee":5966,"source_line":1340,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":157,"record_offset":40904,"parent_record_offset":40880,"ancestor_record_offset":40396,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":157,"record_offset":40880,"parent_record_offset":40856,"ancestor_record_offset":40396,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":157,"record_offset":40484,"parent_record_offset":40460,"ancestor_record_offset":40396,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":157,"record_offset":40532,"parent_record_offset":40508,"ancestor_record_offset":40396,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":157,"record_offset":40508,"parent_record_offset":40484,"ancestor_record_offset":40396,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::canonicalize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":157,"record_offset":40604,"parent_record_offset":40576,"ancestor_record_offset":40396,"inlinee":9323,"source_line":2678,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":157,"record_offset":40828,"parent_record_offset":40800,"ancestor_record_offset":40396,"inlinee":5965,"source_line":253,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":157,"record_offset":40800,"parent_record_offset":40772,"ancestor_record_offset":40396,"inlinee":5963,"source_line":239,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":157,"record_offset":40460,"parent_record_offset":40436,"ancestor_record_offset":40396,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$2::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/panic_report.rs
//     evidence={"module":157,"record_offset":40684,"parent_record_offset":40640,"ancestor_record_offset":40396,"inlinee":13535,"source_line":53,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$2::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 53
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x34DF
// 首条位置: {"module":157,"inlinee":13535}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":157,"record_offset":40716,"parent_record_offset":40684,"ancestor_record_offset":40684,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":157,"record_offset":40744,"parent_record_offset":40684,"ancestor_record_offset":40684,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":157,"record_offset":41168,"parent_record_offset":41144,"ancestor_record_offset":40684,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":157,"record_offset":41144,"parent_record_offset":41056,"ancestor_record_offset":40684,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":157,"record_offset":41056,"parent_record_offset":41028,"ancestor_record_offset":40684,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":157,"record_offset":41088,"parent_record_offset":41056,"ancestor_record_offset":40684,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":157,"record_offset":41112,"parent_record_offset":41088,"ancestor_record_offset":40684,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":157,"record_offset":40772,"parent_record_offset":40744,"ancestor_record_offset":40684,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":157,"record_offset":40856,"parent_record_offset":40828,"ancestor_record_offset":40684,"inlinee":5966,"source_line":1340,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":157,"record_offset":40904,"parent_record_offset":40880,"ancestor_record_offset":40684,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":157,"record_offset":40880,"parent_record_offset":40856,"ancestor_record_offset":40684,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":157,"record_offset":40828,"parent_record_offset":40800,"ancestor_record_offset":40684,"inlinee":5965,"source_line":253,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":157,"record_offset":40800,"parent_record_offset":40772,"ancestor_record_offset":40684,"inlinee":5963,"source_line":239,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 151
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x34E8
// 首条位置: {"module":157,"inlinee":13544}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":157,"record_offset":62748,"parent_record_offset":62640,"ancestor_record_offset":62640,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":157,"record_offset":62664,"parent_record_offset":62640,"ancestor_record_offset":62640,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":157,"record_offset":62772,"parent_record_offset":62748,"ancestor_record_offset":62640,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":157,"record_offset":62712,"parent_record_offset":62688,"ancestor_record_offset":62640,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":157,"record_offset":62688,"parent_record_offset":62664,"ancestor_record_offset":62640,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$4::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 155
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2E7B
// 首条位置: {"module":104,"inlinee":11899}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":104,"record_offset":141448,"parent_record_offset":141428,"ancestor_record_offset":141428,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$5
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 165
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1FB4
// 首条位置: {"module":31,"inlinee":8116}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":31,"record_offset":20428,"parent_record_offset":19520,"ancestor_record_offset":19520,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":31,"record_offset":24732,"parent_record_offset":19520,"ancestor_record_offset":19520,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":31,"record_offset":19696,"parent_record_offset":19520,"ancestor_record_offset":19520,"inlinee":4783,"source_line":386,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":31,"record_offset":20580,"parent_record_offset":19520,"ancestor_record_offset":19520,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":31,"record_offset":20400,"parent_record_offset":19520,"ancestor_record_offset":19520,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":31,"record_offset":23564,"parent_record_offset":19520,"ancestor_record_offset":19520,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":31,"record_offset":19936,"parent_record_offset":19520,"ancestor_record_offset":19520,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":31,"record_offset":23600,"parent_record_offset":19520,"ancestor_record_offset":19520,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_direct] sentry_core::clientoptions::impl$2::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"sentry-core","version":"0.31.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sentry-core-0.31.7/src/clientoptions.rs
//     evidence={"module":31,"record_offset":19908,"parent_record_offset":19520,"ancestor_record_offset":19520,"inlinee":8118,"source_line":230,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_direct] sentry_types::protocol::v7::impl$170::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"sentry-types","version":"0.31.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sentry-types-0.31.7/src/protocol/v7.rs
//     evidence={"module":31,"record_offset":21652,"parent_record_offset":19520,"ancestor_record_offset":19520,"inlinee":8120,"source_line":619,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$30::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":31,"record_offset":22280,"parent_record_offset":19520,"ancestor_record_offset":19520,"inlinee":8124,"source_line":1738,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":31,"record_offset":19808,"parent_record_offset":19784,"ancestor_record_offset":19520,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":31,"record_offset":19832,"parent_record_offset":19808,"ancestor_record_offset":19520,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":31,"record_offset":20160,"parent_record_offset":20136,"ancestor_record_offset":19520,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":31,"record_offset":19752,"parent_record_offset":19724,"ancestor_record_offset":19520,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":31,"record_offset":19784,"parent_record_offset":19752,"ancestor_record_offset":19520,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":31,"record_offset":20136,"parent_record_offset":20080,"ancestor_record_offset":19520,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":31,"record_offset":19724,"parent_record_offset":19696,"ancestor_record_offset":19520,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":31,"record_offset":23656,"parent_record_offset":23632,"ancestor_record_offset":19520,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":31,"record_offset":23704,"parent_record_offset":23680,"ancestor_record_offset":19520,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":31,"record_offset":20840,"parent_record_offset":20812,"ancestor_record_offset":19520,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":31,"record_offset":20108,"parent_record_offset":20080,"ancestor_record_offset":19520,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":31,"record_offset":20080,"parent_record_offset":20056,"ancestor_record_offset":19520,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":31,"record_offset":20900,"parent_record_offset":20868,"ancestor_record_offset":19520,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":31,"record_offset":20868,"parent_record_offset":20840,"ancestor_record_offset":19520,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":31,"record_offset":20056,"parent_record_offset":20032,"ancestor_record_offset":19520,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":31,"record_offset":22724,"parent_record_offset":22688,"ancestor_record_offset":19520,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":31,"record_offset":20776,"parent_record_offset":20748,"ancestor_record_offset":19520,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":31,"record_offset":20748,"parent_record_offset":20720,"ancestor_record_offset":19520,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":31,"record_offset":20692,"parent_record_offset":20664,"ancestor_record_offset":19520,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":31,"record_offset":20720,"parent_record_offset":20692,"ancestor_record_offset":19520,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":31,"record_offset":20664,"parent_record_offset":20636,"ancestor_record_offset":19520,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":31,"record_offset":20636,"parent_record_offset":20608,"ancestor_record_offset":19520,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":31,"record_offset":20608,"parent_record_offset":20580,"ancestor_record_offset":19520,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":31,"record_offset":23836,"parent_record_offset":23800,"ancestor_record_offset":19520,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":31,"record_offset":20268,"parent_record_offset":20244,"ancestor_record_offset":19520,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":31,"record_offset":22496,"parent_record_offset":22452,"ancestor_record_offset":19520,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":31,"record_offset":20812,"parent_record_offset":20776,"ancestor_record_offset":19520,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$11::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":31,"record_offset":22416,"parent_record_offset":22384,"ancestor_record_offset":19520,"inlinee":4623,"source_line":3286,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":31,"record_offset":21800,"parent_record_offset":21768,"ancestor_record_offset":19520,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":31,"record_offset":22452,"parent_record_offset":22416,"ancestor_record_offset":19520,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":31,"record_offset":22748,"parent_record_offset":22724,"ancestor_record_offset":19520,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":31,"record_offset":22772,"parent_record_offset":22748,"ancestor_record_offset":19520,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":31,"record_offset":21052,"parent_record_offset":21028,"ancestor_record_offset":19520,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":31,"record_offset":22820,"parent_record_offset":22796,"ancestor_record_offset":19520,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":31,"record_offset":22796,"parent_record_offset":22772,"ancestor_record_offset":19520,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":31,"record_offset":21968,"parent_record_offset":21940,"ancestor_record_offset":19520,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":31,"record_offset":23680,"parent_record_offset":23656,"ancestor_record_offset":19520,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":31,"record_offset":21028,"parent_record_offset":20776,"ancestor_record_offset":19520,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=38
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":31,"record_offset":19960,"parent_record_offset":19936,"ancestor_record_offset":19520,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":31,"record_offset":23868,"parent_record_offset":23836,"ancestor_record_offset":19520,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":31,"record_offset":19856,"parent_record_offset":19832,"ancestor_record_offset":19520,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":31,"record_offset":20304,"parent_record_offset":20268,"ancestor_record_offset":19520,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":31,"record_offset":20328,"parent_record_offset":20304,"ancestor_record_offset":19520,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":31,"record_offset":20360,"parent_record_offset":20268,"ancestor_record_offset":19520,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] sentry_types::protocol::v7::impl$2::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"sentry-types","version":"0.31.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sentry-types-0.31.7/src/protocol/v7.rs
//     evidence={"module":31,"record_offset":21704,"parent_record_offset":21680,"ancestor_record_offset":19520,"inlinee":8122,"source_line":84,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":31,"record_offset":22316,"parent_record_offset":22280,"ancestor_record_offset":19520,"inlinee":8126,"source_line":642,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":31,"record_offset":22352,"parent_record_offset":22316,"ancestor_record_offset":19520,"inlinee":8128,"source_line":13,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] std::sys_common::wtf8::impl$35::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys_common\wtf8.rs
//     evidence={"module":31,"record_offset":22384,"parent_record_offset":22352,"ancestor_record_offset":19520,"inlinee":8130,"source_line":132,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$5::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/panic_report.rs
//     evidence={"module":31,"record_offset":23632,"parent_record_offset":23600,"ancestor_record_offset":19520,"inlinee":8132,"source_line":210,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$5::closure$0
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 171
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1FD7
// 首条位置: {"module":31,"inlinee":8151}
// 编译实例 RVAs: 0xda930
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: panic_report.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5477041","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369604400,"raw_hex":"e8b1925300","size":5,"soft_fail":false,"target_va":5375081504,"va":5369604458,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: panic_report.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42242671","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369604400,"raw_hex":"e86f928402","size":5,"soft_fail":false,"target_va":5411847168,"va":5369604492,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":31,"record_offset":28492,"parent_record_offset":28464,"ancestor_record_offset":28464,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":31,"record_offset":28664,"parent_record_offset":28640,"ancestor_record_offset":28464,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":31,"record_offset":28640,"parent_record_offset":28616,"ancestor_record_offset":28464,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":31,"record_offset":37616,"parent_record_offset":37592,"ancestor_record_offset":37304,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":31,"record_offset":28616,"parent_record_offset":28592,"ancestor_record_offset":28464,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":31,"record_offset":28592,"parent_record_offset":28572,"ancestor_record_offset":28464,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":31,"record_offset":28512,"parent_record_offset":28492,"ancestor_record_offset":28464,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$5::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 210
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1FC4
// 首条位置: {"module":31,"inlinee":8132}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":31,"record_offset":23656,"parent_record_offset":23632,"ancestor_record_offset":23632,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":31,"record_offset":23704,"parent_record_offset":23680,"ancestor_record_offset":23632,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":31,"record_offset":23680,"parent_record_offset":23656,"ancestor_record_offset":23632,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$6
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 238
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x31B4
// 首条位置: {"module":137,"inlinee":12724}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":137,"record_offset":68532,"parent_record_offset":68508,"ancestor_record_offset":68508,"inlinee":4908,"source_line":1726,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":137,"record_offset":68656,"parent_record_offset":68508,"ancestor_record_offset":68508,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":137,"record_offset":68560,"parent_record_offset":68532,"ancestor_record_offset":68508,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":137,"record_offset":68588,"parent_record_offset":68560,"ancestor_record_offset":68508,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":137,"record_offset":68620,"parent_record_offset":68560,"ancestor_record_offset":68508,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00137.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 9; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

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
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] iced_core::rectangle::Rectangle<f32> (*)(ref$<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >)
// 候选参数类型: ["ref$<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >"]
// [文件级接口候选，具体函数归属 unknown] country_core::render::render_config::VideoMode (*)(ref$<winit::monitor::VideoMode>)
// 候选参数类型: ["ref$<winit::monitor::VideoMode>"]
// [文件级接口候选，具体函数归属 unknown] std::ffi::os_str::OsString (*)(ref$<alloc::string::String>)
// 候选参数类型: ["ref$<alloc::string::String>"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)
// 候选参数类型: ["bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>"]
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<system_wall_constructor::WallPathSegmentationMasksMinusStairs>)
// 候选参数类型: ["bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>", "bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>", "bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<system_wall_constructor::WallPathSegmentationMasksMinusStairs>"]
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<system_wall_constructor::WallPathSegmentationMasksMinusStairs>
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_report::on_game_crashed
// 信息 ID: ea3dcd8228bc9482ffb1db0f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::main @ 0x3dca83 -> 0x3ae940 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$0
// 信息 ID: 46d6233f397b7cc85f85b545
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::panic_reporter::panic_report::on_game_crashed", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::panic_report::on_game_crashed; module=157, inline_record=40196; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$1
// 信息 ID: eda9d7503b3e038feef89a62
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::panic_reporter::panic_report::on_game_crashed", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::panic_report::on_game_crashed; module=157, inline_record=40288; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$2
// 信息 ID: c3e50a5f467d29451bb255dc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "tiny_glade::panic_reporter::panic_report::on_game_crashed", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::panic_report::on_game_crashed; module=157, inline_record=40396; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$2::closure$0
// 信息 ID: 0dae20f55fa2d2d628505545
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "tiny_glade::panic_reporter::panic_report::on_game_crashed", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::panic_report::on_game_crashed; module=157, inline_record=40684; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$4
// 信息 ID: cc3f6c3e5f60400291dcf058
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "tiny_glade::panic_reporter::panic_report::on_game_crashed", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::panic_report::on_game_crashed; module=157, inline_record=62640; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$4::closure$0
// 信息 ID: d3622a4b6e183ee83ef42fe5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "tiny_glade::panic_reporter::panic_report::on_game_crashed", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$4::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<alloc::string::String,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<sentry_types::protocol::v7::Frame>,tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$4::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<alloc::string::String,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<sentry_types::protocol::v7::Frame>,tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$4::closure_env$0> >; module=104, inline_record=141428; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<alloc::string::String,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<sentry_types::protocol::v7::Frame>,tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$4::closure_env$0> >; module=104, inline_record=142116; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$5
// 信息 ID: 3e4d6cba7fa77832f16ca397
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 5, "lexical_owner": "tiny_glade::panic_reporter::panic_report::on_game_crashed", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$5 <- std::sys::backtrace::__rust_begin_short_backtrace<tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$5,enum2$<core::result::Result<alloc::string::String,anyhow::Error> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$5 <- std::thread::spawn<tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$5,enum2$<core::result::Result<alloc::string::String,anyhow::Error> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$5 <- core::ops::function::FnOnce::call_once<std::thread::impl$0::spawn_unchecked_::closure_env$1<tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$5,enum2$<core::result::Result<alloc::string::String,anyhow::Error> > >,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$5 <- core::ptr::drop_in_place<tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$5>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$5 <- core::ptr::drop_in_place<std::thread::impl$0::spawn_unchecked_::closure_env$1<tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$5,enum2$<core::result::Result<alloc::string::String,anyhow::Error> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$5 <- core::ptr::drop_in_place<enum2$<core::option::Option<tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$5> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$5 <- core::ptr::drop_in_place<std::thread::impl$0::spawn_unchecked_::MaybeDangling<tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$5> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 7 条。
// [PDB 内联宿主] std::sys::backtrace::__rust_begin_short_backtrace<tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$5,enum2$<core::result::Result<alloc::string::String,anyhow::Error> > >; module=31, inline_record=19520; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$5::closure$0
// 信息 ID: fda3e4060af2b69e36a236c8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 5, "lexical_owner": "tiny_glade::panic_reporter::panic_report::on_game_crashed", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$5::closure_env$0 <- core::ops::function::FnOnce::call_once<tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$5::closure_env$0,tuple$<sentry_types::protocol::v7::Event> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$5::closure_env$0,tuple$<sentry_types::protocol::v7::Event> >; module=31, inline_record=28464; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$5::closure$1
// 信息 ID: ae354f90895b205e02cf6b06
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 5, "lexical_owner": "tiny_glade::panic_reporter::panic_report::on_game_crashed", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] std::sys::backtrace::__rust_begin_short_backtrace<tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$5,enum2$<core::result::Result<alloc::string::String,anyhow::Error> > >; module=31, inline_record=23632; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$6
// 信息 ID: 745df10c2427caa1b4d57ccc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 6, "lexical_owner": "tiny_glade::panic_reporter::panic_report::on_game_crashed", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$6 <- crash_reporter_gui::run_crash_reporter_gui<tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$6>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$6 <- core::ops::function::FnOnce::call_once<tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$6,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$6 <- core::ptr::drop_in_place<tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$6>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$6,tuple$<> >; module=137, inline_record=68508; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
