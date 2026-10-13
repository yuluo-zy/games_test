// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/tiny-glade/src/panic_reporter/panic_report.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: ff5a568a3f04d0df6dc3237f466c47238de9e2fad6eaabac64a884c23b241d15
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: panic_report.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 4, 14, 68, 73, 205
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"PDB_registry_path_only","version":"0.4.21"}]
// use sentry_types; // namespace candidate
//   package identities: [{"name":"sentry-types","source":"PDB_registry_path_only","version":"0.31.7"}]
// use std; // namespace candidate
//   package identities: []
// use tiny_glade; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_report::on_game_crashed
// 记录形式: compiled_procedure; instances=1
// 源码行: 31
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4,"record_offset":6656,"rva":24096,"va":5368733216}
// 编译实例 RVAs: 0x5e20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$tempfile..dir..TempDir$u20$as$u20$core..ops..drop..Drop$GT$::drop
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t7257907","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368733216,"raw_hex":"e833bf6e00","size":5,"soft_fail":false,"target_va":5375997872,"va":5368739960,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400056b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: panic_report.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11871","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368733216,"raw_hex":"e8a1d1ffff","size":5,"soft_fail":false,"target_va":5368731312,"va":5368743178,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400057e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: panic_report.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11166","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368733216,"raw_hex":"e862d4ffff","size":5,"soft_fail":false,"target_va":5368731616,"va":5368742777,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400058a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: panic_report.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11560","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368733216,"raw_hex":"e8d8d2ffff","size":5,"soft_fail":false,"target_va":5368731808,"va":5368743363,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140005a60
//     owner=unresolved_namespace; count=1
//     target_aliases=118 candidates; complete list: panic_report.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8767","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368733216,"raw_hex":"e8c1ddffff","size":5,"soft_fail":false,"target_va":5368732256,"va":5368741018,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400d6b90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: panic_report.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t849370","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368733216,"raw_hex":"e8daf50c00","size":5,"soft_fail":false,"target_va":5369588624,"va":5368739249,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e16c0
//     owner=unresolved_namespace; count=9
//     target_aliases=4 candidates; complete list: panic_report.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t898945","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368733216,"raw_hex":"e881b70d00","size":5,"soft_fail":false,"target_va":5369632448,"va":5368733498,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e1a00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: panic_report.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t897853","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368733216,"raw_hex":"e83db30d00","size":5,"soft_fail":false,"target_va":5369633280,"va":5368735422,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e6060
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: panic_report.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t912746","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368733216,"raw_hex":"e86aed0d00","size":5,"soft_fail":false,"target_va":5369651296,"va":5368738545,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14035d620
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: panic_report.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3493773","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368733216,"raw_hex":"e88d4f3500","size":5,"soft_fail":false,"target_va":5372237344,"va":5368743566,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403618f0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: panic_report.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3514526","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368733216,"raw_hex":"e89ea03500","size":5,"soft_fail":false,"target_va":5372254448,"va":5368739917,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14036bc70
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: panic_report.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3554248","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368733216,"raw_hex":"e8c83b3600","size":5,"soft_fail":false,"target_va":5372296304,"va":5368742051,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140370260
//     owner=unresolved_namespace; count=9
//     target_aliases=89 candidates; complete list: panic_report.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3579964","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368733216,"raw_hex":"e83ca03600","size":5,"soft_fail":false,"target_va":5372314208,"va":5368734239,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140372930
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: panic_report.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3589307","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368733216,"raw_hex":"e8bbc43600","size":5,"soft_fail":false,"target_va":5372324144,"va":5368734832,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140382c20
//     owner=unresolved_namespace; count=2
//     target_aliases=37 candidates; complete list: panic_report.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3648888","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368733216,"raw_hex":"e878ad3700","size":5,"soft_fail":false,"target_va":5372390432,"va":5368741539,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140389560
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: panic_report.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3673310","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368733216,"raw_hex":"e8de0c3800","size":5,"soft_fail":false,"target_va":5372417376,"va":5368744061,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140389e80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: panic_report.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3676040","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368733216,"raw_hex":"e888173800","size":5,"soft_fail":false,"target_va":5372419712,"va":5368743667,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14038bd00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: panic_report.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3692567","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368733216,"raw_hex":"e817583800","size":5,"soft_fail":false,"target_va":5372427520,"va":5368734948,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=11
//     target_aliases=1 candidates; complete list: panic_report.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4015809","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368733216,"raw_hex":"e8c1463d00","size":5,"soft_fail":false,"target_va":5372749696,"va":5368733882,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=17
//     target_aliases=1 candidates; complete list: panic_report.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4016038","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368733216,"raw_hex":"e8a6473d00","size":5,"soft_fail":false,"target_va":5372749712,"va":5368733669,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406299f0
//     owner=unresolved_namespace; count=14
//     target_aliases=28 candidates; complete list: panic_report.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6436964","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368733216,"raw_hex":"e864386200","size":5,"soft_fail":false,"target_va":5375171056,"va":5368734087,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406f3750
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: panic_report.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7264218","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368733216,"raw_hex":"e8dad76e00","size":5,"soft_fail":false,"target_va":5375997776,"va":5368733553,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140852b50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: panic_report.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8702874","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368733216,"raw_hex":"e89acb8400","size":5,"soft_fail":false,"target_va":5377436496,"va":5368733617,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=14
//     target_aliases=1 candidates; complete list: panic_report.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34859439","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368733216,"raw_hex":"e8afe91302","size":5,"soft_fail":false,"target_va":5403593360,"va":5368733916,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=11
//     evidence={"assembly":"\tcallq\t34237767","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368733216,"raw_hex":"e8476d0a02","size":5,"soft_fail":false,"target_va":5402978144,"va":5368740372,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=10
//     evidence={"assembly":"\tcallq\t8786695","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368733216,"raw_hex":"e807138600","size":5,"soft_fail":false,"target_va":5377527120,"va":5368740420,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::__private::format_err
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t34169498","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368733216,"raw_hex":"e89a620902","size":5,"soft_fail":false,"target_va":5402903312,"va":5368733809,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t8628954","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368733216,"raw_hex":"e8daaa8300","size":5,"soft_fail":false,"target_va":5377364336,"va":5368735377,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34238287","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368733216,"raw_hex":"e84f6f0a02","size":5,"soft_fail":false,"target_va":5402978688,"va":5368740396,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<enum2$<core::result::Result<alloc::string::String,anyhow::Error> >,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-9659","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368733216,"raw_hex":"e845daffff","size":5,"soft_fail":false,"target_va":5368731984,"va":5368741638,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<std::path::PathBuf,alloc::string::String> > >
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t-9324","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368733216,"raw_hex":"e894dbffff","size":5,"soft_fail":false,"target_va":5368732400,"va":5368741719,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<std::path::PathBuf,anyhow::Error> > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-11615","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368733216,"raw_hex":"e8a1d2ffff","size":5,"soft_fail":false,"target_va":5368732208,"va":5368743818,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<tempfile::dir::TempDir,std::io::error::Error> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-9682","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368733216,"raw_hex":"e82edaffff","size":5,"soft_fail":false,"target_va":5368732464,"va":5368742141,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<tiny_glade::crash_pickup::crash_payload::RecoveredSave,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError> > > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-11240","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368733216,"raw_hex":"e818d4ffff","size":5,"soft_fail":false,"target_va":5368731856,"va":5368743091,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::session::SessionMeta::write_to_folder
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5517664","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368733216,"raw_hex":"e860315400","size":5,"soft_fail":false,"target_va":5374255664,"va":5368737995,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::load_last_used_locale
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t7417157","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368733216,"raw_hex":"e8452d7100","size":5,"soft_fail":false,"target_va":5376152288,"va":5368735126,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=9
//     evidence={"assembly":"\tcallq\t8620377","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368733216,"raw_hex":"e859898300","size":5,"soft_fail":false,"target_va":5377353776,"va":5368733394,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirBuilder::_create
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t8671059","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368733216,"raw_hex":"e8534f8400","size":5,"soft_fail":false,"target_va":5377408752,"va":5368737688,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t8705136","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368733216,"raw_hex":"e870d48400","size":5,"soft_fail":false,"target_va":5377439776,"va":5368734635,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::pal::windows::fs::canonicalize
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t8740885","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368733216,"raw_hex":"e815608500","size":5,"soft_fail":false,"target_va":5377474992,"va":5368734102,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::pal::windows::fs::copy
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t8740428","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368733216,"raw_hex":"e84c5e8500","size":5,"soft_fail":false,"target_va":5377475184,"va":5368734751,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::pal::windows::fs::rename
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t8735204","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368733216,"raw_hex":"e8e4498500","size":5,"soft_fail":false,"target_va":5377469888,"va":5368734679,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::pal::windows::fs::stat
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t8735709","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368733216,"raw_hex":"e8dd4b8500","size":5,"soft_fail":false,"target_va":5377473264,"va":5368737550,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::JoinHandle$LT$T$GT$::join
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1927823","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368733216,"raw_hex":"e88f6a1d00","size":5,"soft_fail":false,"target_va":5370667184,"va":5368739356,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_package_integrity::verify_package_manifest
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4401443","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368733216,"raw_hex":"e823294300","size":5,"soft_fail":false,"target_va":5373139696,"va":5368738248,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3324161","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368733216,"raw_hex":"e801b93200","size":5,"soft_fail":false,"target_va":5372062304,"va":5368738138,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::file::rename_file_to_backup
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8205896","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368733216,"raw_hex":"e848367d00","size":5,"soft_fail":false,"target_va":5376946016,"va":5368740115,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*(%rdx)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5368733216,"raw_hex":"ff12","size":2,"soft_fail":false,"va":5368739644,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::rc::Rc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\rc.rs
//     evidence={"module":4,"record_offset":21456,"parent_record_offset":6656,"ancestor_record_offset":6656,"inlinee":4778,"source_line":391,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":4,"record_offset":21136,"parent_record_offset":6656,"ancestor_record_offset":6656,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_direct] core::cell::RefCell::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":4,"record_offset":21428,"parent_record_offset":6656,"ancestor_record_offset":6656,"inlinee":4776,"source_line":820,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_direct] core::cell::RefCell::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":4,"record_offset":21996,"parent_record_offset":6656,"ancestor_record_offset":6656,"inlinee":4780,"source_line":1256,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":4,"record_offset":7052,"parent_record_offset":6656,"ancestor_record_offset":6656,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":4,"record_offset":7080,"parent_record_offset":6656,"ancestor_record_offset":6656,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":4,"record_offset":9408,"parent_record_offset":6656,"ancestor_record_offset":6656,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1_formatted
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":4,"record_offset":11980,"parent_record_offset":6656,"ancestor_record_offset":6656,"inlinee":4755,"source_line":354,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":4,"record_offset":21164,"parent_record_offset":6656,"ancestor_record_offset":6656,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":4,"record_offset":6912,"parent_record_offset":6656,"ancestor_record_offset":6656,"inlinee":4710,"source_line":651,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":4,"record_offset":21192,"parent_record_offset":6656,"ancestor_record_offset":6656,"inlinee":4773,"source_line":1127,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":4,"record_offset":21968,"parent_record_offset":6656,"ancestor_record_offset":6656,"inlinee":4779,"source_line":953,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":4,"record_offset":7916,"parent_record_offset":6656,"ancestor_record_offset":6656,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":4,"record_offset":8244,"parent_record_offset":6656,"ancestor_record_offset":6656,"inlinee":4730,"source_line":1319,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":4,"record_offset":7804,"parent_record_offset":6656,"ancestor_record_offset":6656,"inlinee":4722,"source_line":682,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::is_ok
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":4,"record_offset":10276,"parent_record_offset":6656,"ancestor_record_offset":6656,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":4,"record_offset":7832,"parent_record_offset":6656,"ancestor_record_offset":6656,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":4,"record_offset":8148,"parent_record_offset":6656,"ancestor_record_offset":6656,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":4,"record_offset":20860,"parent_record_offset":6656,"ancestor_record_offset":6656,"inlinee":4772,"source_line":631,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$5::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":4,"record_offset":7136,"parent_record_offset":6656,"ancestor_record_offset":6656,"inlinee":4720,"source_line":1680,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":4,"record_offset":7108,"parent_record_offset":6656,"ancestor_record_offset":6656,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":4,"record_offset":6968,"parent_record_offset":6656,"ancestor_record_offset":6656,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_direct] std::fs::copy
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":4,"record_offset":10332,"parent_record_offset":6656,"ancestor_record_offset":6656,"inlinee":4754,"source_line":2066,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_direct] std::fs::create_dir_all
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":4,"record_offset":18416,"parent_record_offset":6656,"ancestor_record_offset":6656,"inlinee":4768,"source_line":2306,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_direct] std::fs::rename
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":4,"record_offset":9632,"parent_record_offset":6656,"ancestor_record_offset":6656,"inlinee":4748,"source_line":2005,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::exists
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":4,"record_offset":18284,"parent_record_offset":6656,"ancestor_record_offset":6656,"inlinee":4766,"source_line":2862,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":4,"record_offset":9604,"parent_record_offset":6656,"ancestor_record_offset":6656,"inlinee":4747,"source_line":2551,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$44::deref
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":4,"record_offset":9464,"parent_record_offset":6656,"ancestor_record_offset":6656,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::crash_pickup::crash_payload::impl$7::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":4,"record_offset":12064,"parent_record_offset":6656,"ancestor_record_offset":6656,"inlinee":4757,"source_line":26,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4,"record_offset":7548,"parent_record_offset":7524,"ancestor_record_offset":6656,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4,"record_offset":7572,"parent_record_offset":7548,"ancestor_record_offset":6656,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4,"record_offset":8096,"parent_record_offset":8072,"ancestor_record_offset":6656,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4,"record_offset":21528,"parent_record_offset":21492,"ancestor_record_offset":6656,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4,"record_offset":7524,"parent_record_offset":7488,"ancestor_record_offset":6656,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4,"record_offset":8072,"parent_record_offset":8016,"ancestor_record_offset":6656,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":4,"record_offset":21492,"parent_record_offset":21456,"ancestor_record_offset":6656,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":4,"record_offset":22600,"parent_record_offset":22572,"ancestor_record_offset":6656,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4,"record_offset":8044,"parent_record_offset":8016,"ancestor_record_offset":6656,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4,"record_offset":7488,"parent_record_offset":7460,"ancestor_record_offset":6656,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4,"record_offset":7460,"parent_record_offset":7432,"ancestor_record_offset":6656,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4,"record_offset":7644,"parent_record_offset":7460,"ancestor_record_offset":6656,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4,"record_offset":8016,"parent_record_offset":7988,"ancestor_record_offset":6656,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] alloc::rc::RcInnerPtr::dec_strong
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\rc.rs
//     evidence={"module":4,"record_offset":22972,"parent_record_offset":22924,"ancestor_record_offset":6656,"inlinee":4794,"source_line":3363,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] alloc::rc::RcInnerPtr::dec_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\rc.rs
//     evidence={"module":4,"record_offset":23216,"parent_record_offset":22924,"ancestor_record_offset":6656,"inlinee":4795,"source_line":3396,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] alloc::rc::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\rc.rs
//     evidence={"module":4,"record_offset":22924,"parent_record_offset":22892,"ancestor_record_offset":6656,"inlinee":4792,"source_line":2159,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":4,"record_offset":7396,"parent_record_offset":7368,"ancestor_record_offset":6656,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":4,"record_offset":7368,"parent_record_offset":7340,"ancestor_record_offset":6656,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":4,"record_offset":7340,"parent_record_offset":7312,"ancestor_record_offset":6656,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":4,"record_offset":7432,"parent_record_offset":7396,"ancestor_record_offset":6656,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$10::clone
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":4,"record_offset":7312,"parent_record_offset":7280,"ancestor_record_offset":6656,"inlinee":4740,"source_line":2780,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":4,"record_offset":21360,"parent_record_offset":21336,"ancestor_record_offset":6656,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":4,"record_offset":22628,"parent_record_offset":22600,"ancestor_record_offset":6656,"inlinee":4115,"source_line":199,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":4,"record_offset":22136,"parent_record_offset":22112,"ancestor_record_offset":6656,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":4,"record_offset":22296,"parent_record_offset":22272,"ancestor_record_offset":6656,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":4,"record_offset":22272,"parent_record_offset":22248,"ancestor_record_offset":6656,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":4,"record_offset":22068,"parent_record_offset":22028,"ancestor_record_offset":6656,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":4,"record_offset":22028,"parent_record_offset":21996,"ancestor_record_offset":6656,"inlinee":4781,"source_line":871,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":4,"record_offset":22112,"parent_record_offset":22068,"ancestor_record_offset":6656,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":4,"record_offset":22248,"parent_record_offset":22224,"ancestor_record_offset":6656,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::impls::impl$19::clone
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\clone.rs
//     evidence={"module":4,"record_offset":7756,"parent_record_offset":7280,"ancestor_record_offset":6656,"inlinee":4516,"source_line":219,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":4,"record_offset":9656,"parent_record_offset":9632,"ancestor_record_offset":6656,"inlinee":4750,"source_line":701,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":4,"record_offset":8208,"parent_record_offset":8184,"ancestor_record_offset":6656,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":4,"record_offset":7708,"parent_record_offset":7684,"ancestor_record_offset":6656,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":4,"record_offset":21336,"parent_record_offset":21228,"ancestor_record_offset":6656,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":4,"record_offset":22320,"parent_record_offset":22296,"ancestor_record_offset":6656,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":4,"record_offset":22652,"parent_record_offset":22628,"ancestor_record_offset":6656,"inlinee":4117,"source_line":392,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":4,"record_offset":6936,"parent_record_offset":6912,"ancestor_record_offset":6656,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":4,"record_offset":14356,"parent_record_offset":13612,"ancestor_record_offset":6656,"inlinee":4512,"source_line":2008,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":4,"record_offset":7684,"parent_record_offset":7396,"ancestor_record_offset":6656,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=76
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":4,"record_offset":7940,"parent_record_offset":7916,"ancestor_record_offset":6656,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4,"record_offset":21300,"parent_record_offset":21276,"ancestor_record_offset":6656,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":4,"record_offset":22168,"parent_record_offset":22068,"ancestor_record_offset":6656,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":4,"record_offset":7600,"parent_record_offset":7572,"ancestor_record_offset":6656,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":4,"record_offset":22344,"parent_record_offset":22320,"ancestor_record_offset":6656,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":4,"record_offset":18344,"parent_record_offset":18316,"ancestor_record_offset":6656,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":4,"record_offset":8524,"parent_record_offset":8288,"ancestor_record_offset":6656,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$5::clone
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":4,"record_offset":12228,"parent_record_offset":12064,"ancestor_record_offset":6656,"inlinee":4720,"source_line":1680,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":4,"record_offset":21252,"parent_record_offset":21228,"ancestor_record_offset":6656,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":4,"record_offset":21276,"parent_record_offset":21252,"ancestor_record_offset":6656,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":4,"record_offset":6992,"parent_record_offset":6968,"ancestor_record_offset":6656,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":4,"record_offset":7016,"parent_record_offset":6992,"ancestor_record_offset":6656,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::impl$25::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":4,"record_offset":14420,"parent_record_offset":14388,"ancestor_record_offset":6656,"inlinee":4765,"source_line":188,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":4,"record_offset":7224,"parent_record_offset":7196,"ancestor_record_offset":6656,"inlinee":4734,"source_line":605,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":4,"record_offset":8368,"parent_record_offset":8344,"ancestor_record_offset":6656,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":4,"record_offset":8416,"parent_record_offset":8392,"ancestor_record_offset":6656,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":4,"record_offset":8392,"parent_record_offset":8368,"ancestor_record_offset":6656,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::create
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":4,"record_offset":18664,"parent_record_offset":18416,"ancestor_record_offset":6656,"inlinee":4771,"source_line":2561,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::recursive
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":4,"record_offset":18440,"parent_record_offset":18416,"ancestor_record_offset":6656,"inlinee":4770,"source_line":2537,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::canonicalize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":4,"record_offset":8488,"parent_record_offset":8460,"ancestor_record_offset":6656,"inlinee":4744,"source_line":2220,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::metadata
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":4,"record_offset":18316,"parent_record_offset":18284,"ancestor_record_offset":6656,"inlinee":4767,"source_line":1926,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::canonicalize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":4,"record_offset":8460,"parent_record_offset":8288,"ancestor_record_offset":6656,"inlinee":4742,"source_line":2788,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":4,"record_offset":8344,"parent_record_offset":8320,"ancestor_record_offset":6656,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$30::clone
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":4,"record_offset":7196,"parent_record_offset":7136,"ancestor_record_offset":6656,"inlinee":4732,"source_line":1627,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$44::deref
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":4,"record_offset":8320,"parent_record_offset":8288,"ancestor_record_offset":6656,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$56::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":4,"record_offset":7880,"parent_record_offset":7856,"ancestor_record_offset":6656,"inlinee":4727,"source_line":1902,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$79::as_ref
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":4,"record_offset":9680,"parent_record_offset":9656,"ancestor_record_offset":6656,"inlinee":4752,"source_line":3182,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$9::clone
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":4,"record_offset":7252,"parent_record_offset":7224,"ancestor_record_offset":6656,"inlinee":4736,"source_line":12,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] std::sys_common::wtf8::impl$34::clone
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys_common\wtf8.rs
//     evidence={"module":4,"record_offset":7280,"parent_record_offset":7252,"ancestor_record_offset":6656,"inlinee":4738,"source_line":135,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::impl$6::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":4,"record_offset":13576,"parent_record_offset":13524,"ancestor_record_offset":6656,"inlinee":4759,"source_line":17,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::impl$8::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":4,"record_offset":13612,"parent_record_offset":13524,"ancestor_record_offset":6656,"inlinee":4761,"source_line":38,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::impl$9::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":4,"record_offset":14388,"parent_record_offset":14356,"ancestor_record_offset":6656,"inlinee":4763,"source_line":44,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/panic_report.rs
//     evidence={"module":4,"record_offset":7856,"parent_record_offset":7832,"ancestor_record_offset":6656,"inlinee":4725,"source_line":46,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/panic_report.rs
//     evidence={"module":4,"record_offset":8184,"parent_record_offset":8148,"ancestor_record_offset":6656,"inlinee":4729,"source_line":50,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$2
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/panic_report.rs
//     evidence={"module":4,"record_offset":8288,"parent_record_offset":8244,"ancestor_record_offset":6656,"inlinee":4741,"source_line":51,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$2::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/panic_report.rs
//     evidence={"module":4,"record_offset":8564,"parent_record_offset":8524,"ancestor_record_offset":6656,"inlinee":4746,"source_line":51,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$4
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/panic_report.rs
//     evidence={"module":4,"record_offset":21228,"parent_record_offset":21192,"ancestor_record_offset":6656,"inlinee":4774,"source_line":164,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 46
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1275
// 首条位置: {"module":4,"inlinee":4725}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::path::impl$56::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":4,"record_offset":7880,"parent_record_offset":7856,"ancestor_record_offset":7856,"inlinee":4727,"source_line":1902,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 50
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1279
// 首条位置: {"module":4,"inlinee":4729}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":4,"record_offset":8208,"parent_record_offset":8184,"ancestor_record_offset":8184,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 51
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1285
// 首条位置: {"module":4,"inlinee":4741}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":4,"record_offset":8628,"parent_record_offset":8288,"ancestor_record_offset":8288,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":4,"record_offset":8524,"parent_record_offset":8288,"ancestor_record_offset":8288,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::canonicalize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":4,"record_offset":8460,"parent_record_offset":8288,"ancestor_record_offset":8288,"inlinee":4742,"source_line":2788,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$44::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":4,"record_offset":8320,"parent_record_offset":8288,"ancestor_record_offset":8288,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4,"record_offset":8900,"parent_record_offset":8876,"ancestor_record_offset":8288,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4,"record_offset":8876,"parent_record_offset":8812,"ancestor_record_offset":8288,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4,"record_offset":8844,"parent_record_offset":8812,"ancestor_record_offset":8288,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4,"record_offset":8812,"parent_record_offset":8780,"ancestor_record_offset":8288,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":4,"record_offset":8592,"parent_record_offset":8564,"ancestor_record_offset":8288,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":4,"record_offset":8656,"parent_record_offset":8628,"ancestor_record_offset":8288,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":4,"record_offset":8368,"parent_record_offset":8344,"ancestor_record_offset":8288,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":4,"record_offset":8416,"parent_record_offset":8392,"ancestor_record_offset":8288,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":4,"record_offset":8392,"parent_record_offset":8368,"ancestor_record_offset":8288,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::canonicalize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":4,"record_offset":8488,"parent_record_offset":8460,"ancestor_record_offset":8288,"inlinee":4744,"source_line":2220,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":4,"record_offset":8344,"parent_record_offset":8320,"ancestor_record_offset":8288,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$2::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/panic_report.rs
//     evidence={"module":4,"record_offset":8564,"parent_record_offset":8524,"ancestor_record_offset":8288,"inlinee":4746,"source_line":51,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$2::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 51
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x128A
// 首条位置: {"module":4,"inlinee":4746}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":4,"record_offset":8592,"parent_record_offset":8564,"ancestor_record_offset":8564,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 164
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12A6
// 首条位置: {"module":4,"inlinee":4774}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":4,"record_offset":21336,"parent_record_offset":21228,"ancestor_record_offset":21228,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":4,"record_offset":21252,"parent_record_offset":21228,"ancestor_record_offset":21228,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":4,"record_offset":21360,"parent_record_offset":21336,"ancestor_record_offset":21228,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4,"record_offset":21300,"parent_record_offset":21276,"ancestor_record_offset":21228,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":4,"record_offset":21276,"parent_record_offset":21252,"ancestor_record_offset":21228,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$4::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 165
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x21A7
// 首条位置: {"module":73,"inlinee":8615}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::impl$5::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":73,"record_offset":88216,"parent_record_offset":88192,"ancestor_record_offset":88192,"inlinee":4512,"source_line":2008,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$5
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 174
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15CB
// 首条位置: {"module":14,"inlinee":5579}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":14,"record_offset":13896,"parent_record_offset":13032,"ancestor_record_offset":13032,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":14,"record_offset":17644,"parent_record_offset":13032,"ancestor_record_offset":13032,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":14,"record_offset":13208,"parent_record_offset":13032,"ancestor_record_offset":13032,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":14,"record_offset":14048,"parent_record_offset":13032,"ancestor_record_offset":13032,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":14,"record_offset":13868,"parent_record_offset":13032,"ancestor_record_offset":13032,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":14,"record_offset":16320,"parent_record_offset":13032,"ancestor_record_offset":13032,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":14,"record_offset":13420,"parent_record_offset":13032,"ancestor_record_offset":13032,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":14,"record_offset":16356,"parent_record_offset":13032,"ancestor_record_offset":13032,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$30::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":14,"record_offset":15720,"parent_record_offset":13032,"ancestor_record_offset":13032,"inlinee":4732,"source_line":1627,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":14,"record_offset":13320,"parent_record_offset":13296,"ancestor_record_offset":13032,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":14,"record_offset":13344,"parent_record_offset":13320,"ancestor_record_offset":13032,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":14,"record_offset":13624,"parent_record_offset":13600,"ancestor_record_offset":13032,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":14,"record_offset":13264,"parent_record_offset":13236,"ancestor_record_offset":13032,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":14,"record_offset":13296,"parent_record_offset":13264,"ancestor_record_offset":13032,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":14,"record_offset":13600,"parent_record_offset":13544,"ancestor_record_offset":13032,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":14,"record_offset":13236,"parent_record_offset":13208,"ancestor_record_offset":13032,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":14,"record_offset":16412,"parent_record_offset":16388,"ancestor_record_offset":13032,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":14,"record_offset":16460,"parent_record_offset":16436,"ancestor_record_offset":13032,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":14,"record_offset":13572,"parent_record_offset":13544,"ancestor_record_offset":13032,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":14,"record_offset":14336,"parent_record_offset":14308,"ancestor_record_offset":13032,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":14,"record_offset":14308,"parent_record_offset":14280,"ancestor_record_offset":13032,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":14,"record_offset":14484,"parent_record_offset":14308,"ancestor_record_offset":13032,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":14,"record_offset":13544,"parent_record_offset":13516,"ancestor_record_offset":13032,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":14,"record_offset":14244,"parent_record_offset":14216,"ancestor_record_offset":13032,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":14,"record_offset":14216,"parent_record_offset":14188,"ancestor_record_offset":13032,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":14,"record_offset":14160,"parent_record_offset":14132,"ancestor_record_offset":13032,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":14,"record_offset":14188,"parent_record_offset":14160,"ancestor_record_offset":13032,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":14,"record_offset":14132,"parent_record_offset":14104,"ancestor_record_offset":13032,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":14,"record_offset":14104,"parent_record_offset":14076,"ancestor_record_offset":13032,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":14,"record_offset":14076,"parent_record_offset":14048,"ancestor_record_offset":13032,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":14,"record_offset":16592,"parent_record_offset":16556,"ancestor_record_offset":13032,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":14,"record_offset":13736,"parent_record_offset":13708,"ancestor_record_offset":13032,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":14,"record_offset":14280,"parent_record_offset":14244,"ancestor_record_offset":13032,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":14,"record_offset":15848,"parent_record_offset":15816,"ancestor_record_offset":13032,"inlinee":4740,"source_line":2780,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":14,"record_offset":15288,"parent_record_offset":15256,"ancestor_record_offset":13032,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":14,"record_offset":14544,"parent_record_offset":14520,"ancestor_record_offset":13032,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":14,"record_offset":15432,"parent_record_offset":15404,"ancestor_record_offset":13032,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":14,"record_offset":16436,"parent_record_offset":16412,"ancestor_record_offset":13032,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":14,"record_offset":14520,"parent_record_offset":14244,"ancestor_record_offset":13032,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=29
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":14,"record_offset":13444,"parent_record_offset":13420,"ancestor_record_offset":13032,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":14,"record_offset":16624,"parent_record_offset":16592,"ancestor_record_offset":13032,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":14,"record_offset":13368,"parent_record_offset":13344,"ancestor_record_offset":13032,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":14,"record_offset":13772,"parent_record_offset":13736,"ancestor_record_offset":13032,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":14,"record_offset":13796,"parent_record_offset":13772,"ancestor_record_offset":13032,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":14,"record_offset":13828,"parent_record_offset":13736,"ancestor_record_offset":13032,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] sentry_types::protocol::v7::impl$2::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"sentry-types","version":"0.31.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sentry-types-0.31.7/src/protocol/v7.rs
//     evidence={"module":14,"record_offset":15192,"parent_record_offset":15168,"ancestor_record_offset":13032,"inlinee":5581,"source_line":84,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":14,"record_offset":15752,"parent_record_offset":15720,"ancestor_record_offset":13032,"inlinee":4734,"source_line":605,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$9::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":14,"record_offset":15784,"parent_record_offset":15752,"ancestor_record_offset":13032,"inlinee":4736,"source_line":12,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] std::sys_common::wtf8::impl$34::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys_common\wtf8.rs
//     evidence={"module":14,"record_offset":15816,"parent_record_offset":15784,"ancestor_record_offset":13032,"inlinee":4738,"source_line":135,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$5::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/panic_report.rs
//     evidence={"module":14,"record_offset":16388,"parent_record_offset":16356,"ancestor_record_offset":13032,"inlinee":5583,"source_line":219,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$5::closure$0
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 180
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15D0
// 首条位置: {"module":14,"inlinee":5584}
// 编译实例 RVAs: 0x1d7d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: panic_report.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3919745","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368829904,"raw_hex":"e881cf3b00","size":5,"soft_fail":false,"target_va":5372749712,"va":5368829962,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: panic_report.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34763359","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368829904,"raw_hex":"e85f721202","size":5,"soft_fail":false,"target_va":5403593360,"va":5368829996,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":14,"record_offset":18512,"parent_record_offset":18484,"ancestor_record_offset":18484,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":14,"record_offset":18660,"parent_record_offset":18636,"ancestor_record_offset":18484,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":14,"record_offset":18636,"parent_record_offset":18612,"ancestor_record_offset":18484,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":14,"record_offset":21344,"parent_record_offset":21316,"ancestor_record_offset":21048,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":14,"record_offset":18612,"parent_record_offset":18592,"ancestor_record_offset":18484,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":14,"record_offset":18532,"parent_record_offset":18512,"ancestor_record_offset":18484,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$5::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 219
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15CF
// 首条位置: {"module":14,"inlinee":5583}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":14,"record_offset":16412,"parent_record_offset":16388,"ancestor_record_offset":16388,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":14,"record_offset":16460,"parent_record_offset":16436,"ancestor_record_offset":16388,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":14,"record_offset":16436,"parent_record_offset":16412,"ancestor_record_offset":16388,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$6
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 258
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2059
// 首条位置: {"module":68,"inlinee":8281}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":68,"record_offset":11904,"parent_record_offset":11880,"ancestor_record_offset":11880,"inlinee":4134,"source_line":1695,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":68,"record_offset":12028,"parent_record_offset":11880,"ancestor_record_offset":11880,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":68,"record_offset":11932,"parent_record_offset":11904,"ancestor_record_offset":11880,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":68,"record_offset":11960,"parent_record_offset":11932,"ancestor_record_offset":11880,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":68,"record_offset":11992,"parent_record_offset":11932,"ancestor_record_offset":11880,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00068.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 8; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> (*)(rhapsody::raw::physical_device::PhysicalDevice)
// 候选参数类型: ["rhapsody::raw::physical_device::PhysicalDevice"]
// [文件级接口候选，具体函数归属 unknown] country_core::render::render_config::VideoMode (*)(ref$<winit::monitor::VideoMode>)
// 候选参数类型: ["ref$<winit::monitor::VideoMode>"]
// [文件级接口候选，具体函数归属 unknown] std::ffi::os_str::OsString (*)(ref$<alloc::string::String>)
// 候选参数类型: ["ref$<alloc::string::String>"]
// [文件级接口候选，具体函数归属 unknown] alloc::string::String (*)(ref$<winit::monitor::VideoMode>)
// 候选参数类型: ["ref$<winit::monitor::VideoMode>"]
// [文件级接口候选，具体函数归属 unknown] iced_core::rectangle::Rectangle<f32> (*)(ref$<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >)
// 候选参数类型: ["ref$<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(ref$<glam::f32::vec2::Vec2>)
// 候选参数类型: ["ref$<glam::f32::vec2::Vec2>"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_wall_constructor::WallPathSegmentationMasksMinusStairs>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::arches::LintelSegments>)
// 候选参数类型: ["bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>", "bevy_ecs::event::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::ResMut<system_wall_constructor::WallPathSegmentationMasksMinusStairs>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::arches::LintelSegments>"]
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_wall_constructor::WallPathSegmentationMasksMinusStairs>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::arches::LintelSegments>
// [文件级接口候选，具体函数归属 unknown] void (*)()
// 候选参数类型: []

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_report::on_game_crashed
// 信息 ID: e120e1645e45572f7a82ee48
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::main @ 0x39985e -> 0x5e20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$0
// 信息 ID: 1c488d271bfb941b5e0a59f5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::panic_reporter::panic_report::on_game_crashed", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::panic_report::on_game_crashed; module=4, inline_record=7856; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$1
// 信息 ID: ecb82f3039e5071fba8a5a5a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::panic_reporter::panic_report::on_game_crashed", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::panic_report::on_game_crashed; module=4, inline_record=8184; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$2
// 信息 ID: 3d87a9b4fb7b10502d2bb316
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "tiny_glade::panic_reporter::panic_report::on_game_crashed", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::panic_report::on_game_crashed; module=4, inline_record=8288; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$2::closure$0
// 信息 ID: c932efbef7ad22e6d810d90c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "tiny_glade::panic_reporter::panic_report::on_game_crashed", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::panic_report::on_game_crashed; module=4, inline_record=8564; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$4
// 信息 ID: bd46e0718401ed9d6a688e59
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "tiny_glade::panic_reporter::panic_report::on_game_crashed", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::panic_report::on_game_crashed; module=4, inline_record=21228; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$4::closure$0
// 信息 ID: d9d79c336171e6bbbee72207
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "tiny_glade::panic_reporter::panic_report::on_game_crashed", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$4::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<alloc::string::String,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<sentry_types::protocol::v7::Frame>,tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$4::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<alloc::string::String,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<sentry_types::protocol::v7::Frame>,tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$4::closure_env$0> >; module=73, inline_record=88192; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<alloc::string::String,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<sentry_types::protocol::v7::Frame>,tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$4::closure_env$0> >; module=73, inline_record=88888; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$5
// 信息 ID: 93fee5b51edd63a3ff9774d9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 5, "lexical_owner": "tiny_glade::panic_reporter::panic_report::on_game_crashed", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$5 <- core::ptr::drop_in_place<enum2$<core::option::Option<tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$5> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$5 <- std::sys_common::backtrace::__rust_begin_short_backtrace<tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$5,enum2$<core::result::Result<alloc::string::String,anyhow::Error> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$5 <- std::thread::spawn<tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$5,enum2$<core::result::Result<alloc::string::String,anyhow::Error> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$5 <- core::ops::function::FnOnce::call_once<std::thread::impl$0::spawn_unchecked_::closure_env$1<tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$5,enum2$<core::result::Result<alloc::string::String,anyhow::Error> > >,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$5 <- core::ptr::drop_in_place<tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$5>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$5 <- core::ptr::drop_in_place<std::thread::impl$0::spawn_unchecked_::closure_env$1<tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$5,enum2$<core::result::Result<alloc::string::String,anyhow::Error> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$5 <- core::ptr::drop_in_place<std::thread::impl$0::spawn_unchecked_::MaybeDangling<tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$5> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 7 条。
// [PDB 内联宿主] std::sys_common::backtrace::__rust_begin_short_backtrace<tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$5,enum2$<core::result::Result<alloc::string::String,anyhow::Error> > >; module=14, inline_record=13032; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$5::closure$0
// 信息 ID: e36bd2254dcdfa9733c8fcba
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 5, "lexical_owner": "tiny_glade::panic_reporter::panic_report::on_game_crashed", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$5::closure_env$0 <- core::ops::function::FnOnce::call_once<tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$5::closure_env$0,tuple$<sentry_types::protocol::v7::Event> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$5::closure_env$0 <- core::ptr::drop_in_place<tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$5::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$5::closure_env$0,tuple$<sentry_types::protocol::v7::Event> >; module=14, inline_record=18484; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$5::closure$1
// 信息 ID: 8d5e683090e03529995fd1a2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 5, "lexical_owner": "tiny_glade::panic_reporter::panic_report::on_game_crashed", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] std::sys_common::backtrace::__rust_begin_short_backtrace<tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$5,enum2$<core::result::Result<alloc::string::String,anyhow::Error> > >; module=14, inline_record=16388; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_report::on_game_crashed::closure$6
// 信息 ID: 35f3dc67042475502668ba01
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 6, "lexical_owner": "tiny_glade::panic_reporter::panic_report::on_game_crashed", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$6 <- crash_reporter_gui::run_crash_reporter_gui<tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$6>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$6 <- core::ops::function::FnOnce::call_once<tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$6,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$6 <- core::ptr::drop_in_place<tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$6>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$6,tuple$<> >; module=68, inline_record=11880; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
