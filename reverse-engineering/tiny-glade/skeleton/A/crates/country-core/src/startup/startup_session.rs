// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/startup/startup_session.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: e4accf215244b7c31d54072c076d3ee3176495321ec5887ea647c053e756a241
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: startup_session.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 387, 1330, 1431
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"PDB_registry_path_only","version":"0.4.21"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::startup::startup_session::load_game_session
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1330,"record_offset":33284,"rva":15290464,"va":5383999584}
// 编译实例 RVAs: 0xe95060
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$alloc..collections..vec_deque..VecDeque$LT$T$C$A$GT$$u20$as$u20$alloc..collections..vec_deque..spec_extend..SpecExtend$LT$T$C$I$GT$$GT$::spec_extend::h958024a7e5a5ad5b (.llvm.3478840621844800694)
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1839775","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383999584,"raw_hex":"e89f121c00","size":5,"soft_fail":false,"target_va":5385844832,"va":5384005052,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$std..fs..ReadDir$u20$as$u20$core..iter..traits..iterator..Iterator$GT$::next
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-6595044","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383999584,"raw_hex":"e81c5e9bff","size":5,"soft_fail":false,"target_va":5377408080,"va":5384003119,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$std..sys..pal..windows..fs..FindNextFileHandle$u20$as$u20$core..ops..drop..Drop$GT$::drop
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-6538230","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383999584,"raw_hex":"e80a3c9cff","size":5,"soft_fail":false,"target_va":5377465440,"va":5384003665,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402c9630
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12369327","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383999584,"raw_hex":"e8514243ff","size":5,"soft_fail":false,"target_va":5371631152,"va":5384000474,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402cea10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12352474","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383999584,"raw_hex":"e8268443ff","size":5,"soft_fail":false,"target_va":5371652624,"va":5384005093,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402cfa00
//     owner=unresolved_namespace; count=5
//     target_aliases=3 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12346814","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383999584,"raw_hex":"e8429a43ff","size":5,"soft_fail":false,"target_va":5371656704,"va":5384003513,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14036bdf0
//     owner=unresolved_namespace; count=1
//     target_aliases=8 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11714701","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383999584,"raw_hex":"e8733f4dff","size":5,"soft_fail":false,"target_va":5372296688,"va":5384011384,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14036ef60
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11701965","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383999584,"raw_hex":"e833714dff","size":5,"soft_fail":false,"target_va":5372309344,"va":5384011304,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14036efd0
//     owner=unresolved_namespace; count=1
//     target_aliases=27 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11700877","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383999584,"raw_hex":"e873754dff","size":5,"soft_fail":false,"target_va":5372309456,"va":5384010328,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140370260
//     owner=unresolved_namespace; count=5
//     target_aliases=89 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11688192","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383999584,"raw_hex":"e800a74dff","size":5,"soft_fail":false,"target_va":5372314208,"va":5384002395,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11250989","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383999584,"raw_hex":"e8d35254ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5384000680,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=33
//     target_aliases=1 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11250610","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383999584,"raw_hex":"e84e5454ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5384000317,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140504f60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10030711","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383999584,"raw_hex":"e889f166ff","size":5,"soft_fail":false,"target_va":5373972320,"va":5384003026,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406299f0
//     owner=unresolved_namespace; count=10
//     target_aliases=28 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8829166","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383999584,"raw_hex":"e8124779ff","size":5,"soft_fail":false,"target_va":5375171056,"va":5384000217,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406d6d50
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8125327","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383999584,"raw_hex":"e8710484ff","size":5,"soft_fail":false,"target_va":5375880528,"va":5384005850,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406d8000
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8119800","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383999584,"raw_hex":"e8081a84ff","size":5,"soft_fail":false,"target_va":5375885312,"va":5384005107,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406db5c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8108096","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383999584,"raw_hex":"e8c04784ff","size":5,"soft_fail":false,"target_va":5375899072,"va":5384007163,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406f0640
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8022944","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383999584,"raw_hex":"e8609485ff","size":5,"soft_fail":false,"target_va":5375985216,"va":5384008155,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14084b050
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6598299","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383999584,"raw_hex":"e865519bff","size":5,"soft_fail":false,"target_va":5377405008,"va":5384003302,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140e94f80
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8692","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383999584,"raw_hex":"e80cdeffff","size":5,"soft_fail":false,"target_va":5383999360,"va":5384008047,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140eaeb40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t101046","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383999584,"raw_hex":"e8b68a0100","size":5,"soft_fail":false,"target_va":5384104768,"va":5384003717,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141089a90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2043561","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383999584,"raw_hex":"e8a92e1f00","size":5,"soft_fail":false,"target_va":5386050192,"va":5384006626,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19592385","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383999584,"raw_hex":"e8c1f42a01","size":5,"soft_fail":false,"target_va":5403593360,"va":5384000970,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t18970415","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383999584,"raw_hex":"e82f772101","size":5,"soft_fail":false,"target_va":5402978144,"va":5384007724,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t18912078","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383999584,"raw_hex":"e84e932001","size":5,"soft_fail":false,"target_va":5402919072,"va":5384006989,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t-12695098","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383999584,"raw_hex":"e8c6493eff","size":5,"soft_fail":false,"target_va":5371305056,"va":5384000149,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-6480670","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383999584,"raw_hex":"e8e21c9dff","size":5,"soft_fail":false,"target_va":5377527120,"va":5384007785,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-6642103","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383999584,"raw_hex":"e849a69aff","size":5,"soft_fail":false,"target_va":5377364336,"va":5384006434,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::world::command_queue::CommandQueue::get_raw
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-6753201","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383999584,"raw_hex":"e84ff498ff","size":5,"soft_fail":false,"target_va":5377246528,"va":5383999724,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t18971243","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383999584,"raw_hex":"e86b7a2101","size":5,"soft_fail":false,"target_va":5402978912,"va":5384007664,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t18971465","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383999584,"raw_hex":"e8497b2101","size":5,"soft_fail":false,"target_va":5402978880,"va":5384007410,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t18971761","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383999584,"raw_hex":"e8717c2101","size":5,"soft_fail":false,"target_va":5402979152,"va":5384007386,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t18971456","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383999584,"raw_hex":"e8407b2101","size":5,"soft_fail":false,"target_va":5402978944,"va":5384007483,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<enum2$<country_core::startup::startup_session::SessionLoadAttempt> > > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-11660","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383999584,"raw_hex":"e874d2ffff","size":5,"soft_fail":false,"target_va":5383999216,"va":5384010871,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-11260465","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383999584,"raw_hex":"e8cf2d54ff","size":5,"soft_fail":false,"target_va":5372749776,"va":5384010236,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<country_core::systems::save::load_game::PostGameLoadCommand> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-9517","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383999584,"raw_hex":"e8d3daffff","size":5,"soft_fail":false,"target_va":5383999488,"va":5384009000,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18972578","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383999584,"raw_hex":"e8a27f2101","size":5,"soft_fail":false,"target_va":5402980128,"va":5384007545,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::session::SessionId::save_folder_path
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-9745026","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383999584,"raw_hex":"e87e4d6bff","size":5,"soft_fail":false,"target_va":5374255184,"va":5384000205,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::session::SessionMeta::try_load_from_path
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-9750386","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383999584,"raw_hex":"e88e386bff","size":5,"soft_fail":false,"target_va":5374255728,"va":5384006109,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::save::load_completion_checker::is_session_ok_to_load
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-8210701","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383999584,"raw_hex":"e8f3b682ff","size":5,"soft_fail":false,"target_va":5375789248,"va":5383999944,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=7
//     evidence={"assembly":"\tcallq\t-6646329","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383999584,"raw_hex":"e8c7959aff","size":5,"soft_fail":false,"target_va":5377353776,"va":5384000100,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirEntry::path
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-6594981","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383999584,"raw_hex":"e85b5e9bff","size":5,"soft_fail":false,"target_va":5377408224,"va":5384003200,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t-6560480","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383999584,"raw_hex":"e820e59bff","size":5,"soft_fail":false,"target_va":5377439776,"va":5384000251,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::file_name
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-6565997","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383999584,"raw_hex":"e893cf9bff","size":5,"soft_fail":false,"target_va":5377437280,"va":5384003272,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::is_dir
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-6562115","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383999584,"raw_hex":"e8bdde9bff","size":5,"soft_fail":false,"target_va":5377441120,"va":5384003230,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::parent
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-6567524","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383999584,"raw_hex":"e89cc99bff","size":5,"soft_fail":false,"target_va":5377436944,"va":5384004463,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::pal::windows::fs::readdir
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-6533739","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383999584,"raw_hex":"e8954d9cff","size":5,"soft_fail":false,"target_va":5377468224,"va":5384001958,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::pal::windows::fs::stat
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-6528909","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383999584,"raw_hex":"e873609cff","size":5,"soft_fail":false,"target_va":5377473264,"va":5384002168,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] uuid::v4::_$LT$impl$u20$uuid..Uuid$GT$::new_v4
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-7649618","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383999584,"raw_hex":"e8ae468bff","size":5,"soft_fail":false,"target_va":5376352096,"va":5384001709,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1330,"record_offset":46972,"parent_record_offset":33284,"ancestor_record_offset":33284,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1330,"record_offset":34576,"parent_record_offset":33284,"ancestor_record_offset":33284,"inlinee":12812,"source_line":542,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::pop_front
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1330,"record_offset":39004,"parent_record_offset":33284,"ancestor_record_offset":33284,"inlinee":10716,"source_line":1610,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::push_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1330,"record_offset":35880,"parent_record_offset":33284,"ancestor_record_offset":33284,"inlinee":10291,"source_line":1685,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1330,"record_offset":47124,"parent_record_offset":33284,"ancestor_record_offset":33284,"inlinee":4441,"source_line":461,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::sort
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1330,"record_offset":42668,"parent_record_offset":33284,"ancestor_record_offset":33284,"inlinee":8700,"source_line":207,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1330,"record_offset":42640,"parent_record_offset":33284,"ancestor_record_offset":33284,"inlinee":7328,"source_line":2218,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1330,"record_offset":39964,"parent_record_offset":33284,"ancestor_record_offset":33284,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1330,"record_offset":40628,"parent_record_offset":33284,"ancestor_record_offset":33284,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1330,"record_offset":42752,"parent_record_offset":33284,"ancestor_record_offset":33284,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1330,"record_offset":45972,"parent_record_offset":33284,"ancestor_record_offset":33284,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":1330,"record_offset":34516,"parent_record_offset":33284,"ancestor_record_offset":33284,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventWriter::send
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":1330,"record_offset":37076,"parent_record_offset":33284,"ancestor_record_offset":33284,"inlinee":7450,"source_line":624,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventWriter::send_default
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":1330,"record_offset":34800,"parent_record_offset":33284,"ancestor_record_offset":33284,"inlinee":7452,"source_line":641,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::insert_resource
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":1330,"record_offset":50696,"parent_record_offset":33284,"ancestor_record_offset":33284,"inlinee":12145,"source_line":598,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::remove_resource
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":1330,"record_offset":33604,"parent_record_offset":33284,"ancestor_record_offset":33284,"inlinee":12139,"source_line":622,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1330,"record_offset":40392,"parent_record_offset":33284,"ancestor_record_offset":33284,"inlinee":4576,"source_line":263,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1330,"record_offset":34716,"parent_record_offset":33284,"ancestor_record_offset":33284,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1330,"record_offset":37676,"parent_record_offset":33284,"ancestor_record_offset":33284,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1330,"record_offset":34744,"parent_record_offset":33284,"ancestor_record_offset":33284,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::nth
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1330,"record_offset":42812,"parent_record_offset":33284,"ancestor_record_offset":33284,"inlinee":11441,"source_line":357,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_direct] core::mem::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1330,"record_offset":37332,"parent_record_offset":33284,"ancestor_record_offset":33284,"inlinee":5326,"source_line":845,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1330,"record_offset":42724,"parent_record_offset":33284,"ancestor_record_offset":33284,"inlinee":4907,"source_line":1680,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1330,"record_offset":45880,"parent_record_offset":33284,"ancestor_record_offset":33284,"inlinee":5324,"source_line":893,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1330,"record_offset":46800,"parent_record_offset":33284,"ancestor_record_offset":33284,"inlinee":4710,"source_line":651,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1330,"record_offset":44936,"parent_record_offset":33284,"ancestor_record_offset":33284,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::take
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1330,"record_offset":37448,"parent_record_offset":33284,"ancestor_record_offset":33284,"inlinee":4134,"source_line":1695,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1330,"record_offset":40300,"parent_record_offset":33284,"ancestor_record_offset":33284,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=30
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1330,"record_offset":39284,"parent_record_offset":33284,"ancestor_record_offset":33284,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1330,"record_offset":39892,"parent_record_offset":33284,"ancestor_record_offset":33284,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1330,"record_offset":49668,"parent_record_offset":33284,"ancestor_record_offset":33284,"inlinee":6160,"source_line":1428,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::first
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1330,"record_offset":34604,"parent_record_offset":33284,"ancestor_record_offset":33284,"inlinee":12681,"source_line":176,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_direct] country_core::meta_commander::MetaCommandFence::unblock
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/meta_commander.rs
//     evidence={"module":1330,"record_offset":46236,"parent_record_offset":33284,"ancestor_record_offset":33284,"inlinee":26612,"source_line":109,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_direct] country_core::meta_commander::MetaCommander::extend_script
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/meta_commander.rs
//     evidence={"module":1330,"record_offset":46124,"parent_record_offset":33284,"ancestor_record_offset":33284,"inlinee":25472,"source_line":137,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::impl$27::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":1330,"record_offset":36204,"parent_record_offset":33284,"ancestor_record_offset":33284,"inlinee":62082,"source_line":225,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::startup_session::MaybeExists::get
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/startup_session.rs
//     evidence={"module":1330,"record_offset":38060,"parent_record_offset":33284,"ancestor_record_offset":33284,"inlinee":62084,"source_line":35,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::startup_session::SessionLoadAttempt::from_session_id
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/startup_session.rs
//     evidence={"module":1330,"record_offset":35064,"parent_record_offset":33284,"ancestor_record_offset":33284,"inlinee":62080,"source_line":64,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::startup_session::load_session_meta
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/startup_session.rs
//     evidence={"module":1330,"record_offset":48108,"parent_record_offset":33284,"ancestor_record_offset":33284,"inlinee":62087,"source_line":356,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":1330,"record_offset":34772,"parent_record_offset":33284,"ancestor_record_offset":33284,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":1330,"record_offset":34632,"parent_record_offset":33284,"ancestor_record_offset":33284,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_direct] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":1330,"record_offset":40336,"parent_record_offset":33284,"ancestor_record_offset":33284,"inlinee":6730,"source_line":812,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_dir
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":1330,"record_offset":37992,"parent_record_offset":33284,"ancestor_record_offset":33284,"inlinee":8478,"source_line":2465,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::exists
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":1330,"record_offset":44644,"parent_record_offset":33284,"ancestor_record_offset":33284,"inlinee":4766,"source_line":2862,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":1330,"record_offset":39864,"parent_record_offset":33284,"ancestor_record_offset":33284,"inlinee":4747,"source_line":2551,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$44::deref
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":1330,"record_offset":40020,"parent_record_offset":33284,"ancestor_record_offset":33284,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1330,"record_offset":36652,"parent_record_offset":36628,"ancestor_record_offset":33284,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1330,"record_offset":36676,"parent_record_offset":36652,"ancestor_record_offset":33284,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=33
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1330,"record_offset":35516,"parent_record_offset":35492,"ancestor_record_offset":33284,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1330,"record_offset":36628,"parent_record_offset":36592,"ancestor_record_offset":33284,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=33
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1330,"record_offset":35492,"parent_record_offset":35436,"ancestor_record_offset":33284,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::buffer_read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1330,"record_offset":39180,"parent_record_offset":39004,"ancestor_record_offset":33284,"inlinee":10717,"source_line":165,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::buffer_write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1330,"record_offset":36100,"parent_record_offset":35880,"ancestor_record_offset":33284,"inlinee":6132,"source_line":171,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1330,"record_offset":35944,"parent_record_offset":35920,"ancestor_record_offset":33284,"inlinee":7711,"source_line":751,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1330,"record_offset":39044,"parent_record_offset":39004,"ancestor_record_offset":33284,"inlinee":11193,"source_line":1254,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::is_full
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1330,"record_offset":35920,"parent_record_offset":35880,"ancestor_record_offset":33284,"inlinee":7710,"source_line":188,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1330,"record_offset":36124,"parent_record_offset":36100,"ancestor_record_offset":33284,"inlinee":7712,"source_line":159,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1330,"record_offset":36004,"parent_record_offset":35880,"ancestor_record_offset":33284,"inlinee":7688,"source_line":200,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1330,"record_offset":36028,"parent_record_offset":36004,"ancestor_record_offset":33284,"inlinee":7689,"source_line":195,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1330,"record_offset":46172,"parent_record_offset":46148,"ancestor_record_offset":33284,"inlinee":59449,"source_line":2798,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::impl$18::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1330,"record_offset":46148,"parent_record_offset":46124,"ancestor_record_offset":33284,"inlinee":11902,"source_line":2825,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::into_iter::IntoIter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\into_iter.rs
//     evidence={"module":1330,"record_offset":46196,"parent_record_offset":46172,"ancestor_record_offset":33284,"inlinee":59451,"source_line":25,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1330,"record_offset":33832,"parent_record_offset":33804,"ancestor_record_offset":33284,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=32
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1330,"record_offset":35464,"parent_record_offset":35436,"ancestor_record_offset":33284,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1330,"record_offset":33804,"parent_record_offset":33768,"ancestor_record_offset":33284,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1330,"record_offset":33932,"parent_record_offset":33908,"ancestor_record_offset":33284,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1330,"record_offset":33768,"parent_record_offset":33736,"ancestor_record_offset":33284,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1330,"record_offset":36592,"parent_record_offset":36564,"ancestor_record_offset":33284,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1330,"record_offset":36564,"parent_record_offset":36536,"ancestor_record_offset":33284,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1330,"record_offset":36748,"parent_record_offset":36564,"ancestor_record_offset":33284,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=33
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1330,"record_offset":35436,"parent_record_offset":35408,"ancestor_record_offset":33284,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1330,"record_offset":36496,"parent_record_offset":36460,"ancestor_record_offset":33284,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1330,"record_offset":47148,"parent_record_offset":47124,"ancestor_record_offset":33284,"inlinee":4442,"source_line":100,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1330,"record_offset":36460,"parent_record_offset":36424,"ancestor_record_offset":33284,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1330,"record_offset":36424,"parent_record_offset":36388,"ancestor_record_offset":33284,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::stable_sort
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1330,"record_offset":42692,"parent_record_offset":42668,"ancestor_record_offset":33284,"inlinee":6490,"source_line":842,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1330,"record_offset":41164,"parent_record_offset":41128,"ancestor_record_offset":33284,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1330,"record_offset":41128,"parent_record_offset":41104,"ancestor_record_offset":33284,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1330,"record_offset":33908,"parent_record_offset":33688,"ancestor_record_offset":33284,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1330,"record_offset":47172,"parent_record_offset":47148,"ancestor_record_offset":33284,"inlinee":4443,"source_line":819,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1330,"record_offset":33880,"parent_record_offset":33688,"ancestor_record_offset":33284,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1330,"record_offset":34924,"parent_record_offset":34888,"ancestor_record_offset":33284,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1330,"record_offset":33736,"parent_record_offset":33688,"ancestor_record_offset":33284,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1330,"record_offset":34048,"parent_record_offset":33688,"ancestor_record_offset":33284,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1330,"record_offset":36536,"parent_record_offset":36496,"ancestor_record_offset":33284,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1330,"record_offset":36388,"parent_record_offset":36348,"ancestor_record_offset":33284,"inlinee":4740,"source_line":2780,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1330,"record_offset":43368,"parent_record_offset":43344,"ancestor_record_offset":33284,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1330,"record_offset":43756,"parent_record_offset":43732,"ancestor_record_offset":33284,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::advance_by
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1330,"record_offset":42844,"parent_record_offset":42812,"ancestor_record_offset":33284,"inlinee":51600,"source_line":233,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1330,"record_offset":43240,"parent_record_offset":42812,"ancestor_record_offset":33284,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=8
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":1330,"record_offset":34544,"parent_record_offset":34516,"ancestor_record_offset":33284,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":1330,"record_offset":34824,"parent_record_offset":34800,"ancestor_record_offset":33284,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::send
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":1330,"record_offset":34888,"parent_record_offset":34824,"ancestor_record_offset":33284,"inlinee":7451,"source_line":231,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::push
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":1330,"record_offset":33636,"parent_record_offset":33604,"ancestor_record_offset":33284,"inlinee":12140,"source_line":501,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::CommandQueue::push
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":1330,"record_offset":34080,"parent_record_offset":33636,"ancestor_record_offset":33284,"inlinee":12149,"source_line":78,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::RawCommandQueue::push
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":1330,"record_offset":33688,"parent_record_offset":33636,"ancestor_record_offset":33284,"inlinee":12142,"source_line":157,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::impls::impl$19::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\clone.rs
//     evidence={"module":1330,"record_offset":36860,"parent_record_offset":36348,"ancestor_record_offset":33284,"inlinee":4516,"source_line":219,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1330,"record_offset":49828,"parent_record_offset":49708,"ancestor_record_offset":33284,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1330,"record_offset":40476,"parent_record_offset":40448,"ancestor_record_offset":33284,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1_formatted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1330,"record_offset":49856,"parent_record_offset":49708,"ancestor_record_offset":33284,"inlinee":4755,"source_line":354,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1330,"record_offset":34012,"parent_record_offset":33988,"ancestor_record_offset":33284,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1330,"record_offset":37356,"parent_record_offset":37332,"ancestor_record_offset":33284,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1330,"record_offset":43860,"parent_record_offset":43832,"ancestor_record_offset":33284,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1330,"record_offset":36052,"parent_record_offset":36028,"ancestor_record_offset":33284,"inlinee":7690,"source_line":1639,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1330,"record_offset":46824,"parent_record_offset":46800,"ancestor_record_offset":33284,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1330,"record_offset":40420,"parent_record_offset":40392,"ancestor_record_offset":33284,"inlinee":4556,"source_line":2154,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$15::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1330,"record_offset":40448,"parent_record_offset":40420,"ancestor_record_offset":33284,"inlinee":4558,"source_line":2174,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1330,"record_offset":36788,"parent_record_offset":36496,"ancestor_record_offset":33284,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=204
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1330,"record_offset":35276,"parent_record_offset":35064,"ancestor_record_offset":33284,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1330,"record_offset":42780,"parent_record_offset":42752,"ancestor_record_offset":33284,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_unaligned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1330,"record_offset":33964,"parent_record_offset":33688,"ancestor_record_offset":33284,"inlinee":12143,"source_line":1546,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1330,"record_offset":43268,"parent_record_offset":43240,"ancestor_record_offset":33284,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1330,"record_offset":41196,"parent_record_offset":41164,"ancestor_record_offset":33284,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1330,"record_offset":46004,"parent_record_offset":45972,"ancestor_record_offset":33284,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1330,"record_offset":37384,"parent_record_offset":37356,"ancestor_record_offset":33284,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1330,"record_offset":36704,"parent_record_offset":36676,"ancestor_record_offset":33284,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1330,"record_offset":35020,"parent_record_offset":34924,"ancestor_record_offset":33284,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write_unaligned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1330,"record_offset":33988,"parent_record_offset":33964,"ancestor_record_offset":33284,"inlinee":12144,"source_line":1579,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1330,"record_offset":38020,"parent_record_offset":37992,"ancestor_record_offset":33284,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1330,"record_offset":40360,"parent_record_offset":40336,"ancestor_record_offset":33284,"inlinee":4772,"source_line":631,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1330,"record_offset":40548,"parent_record_offset":40524,"ancestor_record_offset":33284,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1330,"record_offset":40572,"parent_record_offset":40548,"ancestor_record_offset":33284,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1330,"record_offset":40500,"parent_record_offset":40476,"ancestor_record_offset":33284,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1330,"record_offset":46260,"parent_record_offset":46236,"ancestor_record_offset":33284,"inlinee":6715,"source_line":637,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1330,"record_offset":46404,"parent_record_offset":46368,"ancestor_record_offset":33284,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1330,"record_offset":34656,"parent_record_offset":34632,"ancestor_record_offset":33284,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1330,"record_offset":34680,"parent_record_offset":34656,"ancestor_record_offset":33284,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1330,"record_offset":46284,"parent_record_offset":46260,"ancestor_record_offset":33284,"inlinee":5323,"source_line":3288,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1330,"record_offset":41224,"parent_record_offset":41164,"ancestor_record_offset":33284,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1330,"record_offset":41256,"parent_record_offset":41128,"ancestor_record_offset":33284,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::impl$14::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":1330,"record_offset":50496,"parent_record_offset":50472,"ancestor_record_offset":33284,"inlinee":12063,"source_line":28,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::startup_session::load_game_session::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/startup_session.rs
//     evidence={"module":1330,"record_offset":49708,"parent_record_offset":49668,"ancestor_record_offset":33284,"inlinee":62088,"source_line":194,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::startup_session::load_game_session::closure$2
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/startup_session.rs
//     evidence={"module":1330,"record_offset":44964,"parent_record_offset":44936,"ancestor_record_offset":33284,"inlinee":62085,"source_line":302,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":1330,"record_offset":49884,"parent_record_offset":49708,"ancestor_record_offset":33284,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":1330,"record_offset":49744,"parent_record_offset":49708,"ancestor_record_offset":33284,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":1330,"record_offset":36276,"parent_record_offset":36240,"ancestor_record_offset":33284,"inlinee":4734,"source_line":605,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":1330,"record_offset":35152,"parent_record_offset":35128,"ancestor_record_offset":33284,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":1330,"record_offset":35200,"parent_record_offset":35176,"ancestor_record_offset":33284,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":1330,"record_offset":35176,"parent_record_offset":35152,"ancestor_record_offset":33284,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::metadata
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":1330,"record_offset":38296,"parent_record_offset":38260,"ancestor_record_offset":33284,"inlinee":4767,"source_line":1926,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::exists
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":1330,"record_offset":38260,"parent_record_offset":38060,"ancestor_record_offset":33284,"inlinee":4766,"source_line":2862,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::join
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":1330,"record_offset":35244,"parent_record_offset":35064,"ancestor_record_offset":33284,"inlinee":4747,"source_line":2551,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":1330,"record_offset":35128,"parent_record_offset":35104,"ancestor_record_offset":33284,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$30::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":1330,"record_offset":36240,"parent_record_offset":36204,"ancestor_record_offset":33284,"inlinee":4732,"source_line":1627,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$44::deref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":1330,"record_offset":35104,"parent_record_offset":35064,"ancestor_record_offset":33284,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$9::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":1330,"record_offset":36312,"parent_record_offset":36276,"ancestor_record_offset":33284,"inlinee":4736,"source_line":12,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] std::sys_common::wtf8::impl$34::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys_common\wtf8.rs
//     evidence={"module":1330,"record_offset":36348,"parent_record_offset":36312,"ancestor_record_offset":33284,"inlinee":4738,"source_line":135,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::startup_session::MaybeExists::get
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 35
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF284
// 首条位置: {"module":1330,"inlinee":62084}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1330,"record_offset":38400,"parent_record_offset":38060,"ancestor_record_offset":38060,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::exists
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":1330,"record_offset":38260,"parent_record_offset":38060,"ancestor_record_offset":38060,"inlinee":4766,"source_line":2862,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$44::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":1330,"record_offset":38120,"parent_record_offset":38060,"ancestor_record_offset":38060,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1330,"record_offset":38640,"parent_record_offset":38616,"ancestor_record_offset":38060,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1330,"record_offset":38616,"parent_record_offset":38560,"ancestor_record_offset":38060,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1330,"record_offset":38588,"parent_record_offset":38560,"ancestor_record_offset":38060,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1330,"record_offset":38560,"parent_record_offset":38532,"ancestor_record_offset":38060,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1330,"record_offset":38368,"parent_record_offset":38260,"ancestor_record_offset":38060,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1330,"record_offset":38328,"parent_record_offset":38296,"ancestor_record_offset":38060,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":1330,"record_offset":38168,"parent_record_offset":38144,"ancestor_record_offset":38060,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":1330,"record_offset":38216,"parent_record_offset":38192,"ancestor_record_offset":38060,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":1330,"record_offset":38192,"parent_record_offset":38168,"ancestor_record_offset":38060,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::metadata
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":1330,"record_offset":38296,"parent_record_offset":38260,"ancestor_record_offset":38060,"inlinee":4767,"source_line":1926,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":1330,"record_offset":38144,"parent_record_offset":38120,"ancestor_record_offset":38060,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::startup_session::SessionLoadAttempt::from_session_id
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 64
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF280
// 首条位置: {"module":1330,"inlinee":62080}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1330,"record_offset":35276,"parent_record_offset":35064,"ancestor_record_offset":35064,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":1330,"record_offset":35244,"parent_record_offset":35064,"ancestor_record_offset":35064,"inlinee":4747,"source_line":2551,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$44::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":1330,"record_offset":35104,"parent_record_offset":35064,"ancestor_record_offset":35064,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1330,"record_offset":35516,"parent_record_offset":35492,"ancestor_record_offset":35064,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1330,"record_offset":35492,"parent_record_offset":35436,"ancestor_record_offset":35064,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1330,"record_offset":35464,"parent_record_offset":35436,"ancestor_record_offset":35064,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1330,"record_offset":35436,"parent_record_offset":35408,"ancestor_record_offset":35064,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1330,"record_offset":35300,"parent_record_offset":35276,"ancestor_record_offset":35064,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":1330,"record_offset":35152,"parent_record_offset":35128,"ancestor_record_offset":35064,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":1330,"record_offset":35200,"parent_record_offset":35176,"ancestor_record_offset":35064,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":1330,"record_offset":35176,"parent_record_offset":35152,"ancestor_record_offset":35064,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":1330,"record_offset":35128,"parent_record_offset":35104,"ancestor_record_offset":35064,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::startup_session::load_game_session::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 194
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF288
// 首条位置: {"module":1330,"inlinee":62088}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1330,"record_offset":49828,"parent_record_offset":49708,"ancestor_record_offset":49708,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1_formatted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1330,"record_offset":49856,"parent_record_offset":49708,"ancestor_record_offset":49708,"inlinee":4755,"source_line":354,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1330,"record_offset":49912,"parent_record_offset":49708,"ancestor_record_offset":49708,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":1330,"record_offset":49884,"parent_record_offset":49708,"ancestor_record_offset":49708,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":1330,"record_offset":49744,"parent_record_offset":49708,"ancestor_record_offset":49708,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1330,"record_offset":49768,"parent_record_offset":49744,"ancestor_record_offset":49708,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1330,"record_offset":49792,"parent_record_offset":49768,"ancestor_record_offset":49708,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::startup_session::load_game_session::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 261
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5556
// 首条位置: {"module":387,"inlinee":21846}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":387,"record_offset":3444,"parent_record_offset":3424,"ancestor_record_offset":3424,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":387,"record_offset":3492,"parent_record_offset":3468,"ancestor_record_offset":3424,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":387,"record_offset":3468,"parent_record_offset":3444,"ancestor_record_offset":3424,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::startup_session::load_game_session::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 302
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF285
// 首条位置: {"module":1330,"inlinee":62085}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":1330,"record_offset":44988,"parent_record_offset":44964,"ancestor_record_offset":44964,"inlinee":4747,"source_line":2551,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::startup_session::load_session_meta
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 356
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF287
// 首条位置: {"module":1330,"inlinee":62087}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1330,"record_offset":48464,"parent_record_offset":48108,"ancestor_record_offset":48108,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":1330,"record_offset":48292,"parent_record_offset":48108,"ancestor_record_offset":48108,"inlinee":4747,"source_line":2551,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$44::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":1330,"record_offset":48152,"parent_record_offset":48108,"ancestor_record_offset":48108,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1330,"record_offset":48704,"parent_record_offset":48680,"ancestor_record_offset":48108,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1330,"record_offset":48680,"parent_record_offset":48624,"ancestor_record_offset":48108,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1330,"record_offset":48652,"parent_record_offset":48624,"ancestor_record_offset":48108,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1330,"record_offset":48624,"parent_record_offset":48596,"ancestor_record_offset":48108,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1330,"record_offset":48488,"parent_record_offset":48464,"ancestor_record_offset":48108,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":1330,"record_offset":48200,"parent_record_offset":48176,"ancestor_record_offset":48108,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":1330,"record_offset":48248,"parent_record_offset":48224,"ancestor_record_offset":48108,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":1330,"record_offset":48224,"parent_record_offset":48200,"ancestor_record_offset":48108,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":1330,"record_offset":48176,"parent_record_offset":48152,"ancestor_record_offset":48108,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::startup_session::copy_dir_all<std::path::PathBuf,std::path::PathBuf>
// 记录形式: compiled_procedure; instances=1
// 源码行: 362
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1431,"record_offset":119092,"rva":17341072,"va":5386050192}
// 编译实例 RVAs: 0x1089a90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$std..fs..ReadDir$u20$as$u20$core..iter..traits..iterator..Iterator$GT$::next
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-8642480","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386050192,"raw_hex":"e850207cff","size":5,"soft_fail":false,"target_va":5377408080,"va":5386050555,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$std..sys..pal..windows..fs..FindNextFileHandle$u20$as$u20$core..ops..drop..Drop$GT$::drop
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-8585591","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386050192,"raw_hex":"e889fe7cff","size":5,"soft_fail":false,"target_va":5377465440,"va":5386051026,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140268bb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14815732","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386050192,"raw_hex":"e80cee1dff","size":5,"soft_fail":false,"target_va":5371235248,"va":5386050975,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402cfa00
//     owner=unresolved_namespace; count=7
//     target_aliases=3 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14394310","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386050192,"raw_hex":"e83a5c24ff","size":5,"soft_fail":false,"target_va":5371656704,"va":5386051009,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14036ef60
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13742904","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386050192,"raw_hex":"e8c84c2eff","size":5,"soft_fail":false,"target_va":5372309344,"va":5386052243,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=12
//     target_aliases=1 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13300743","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386050192,"raw_hex":"e8f90b35ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5386050450,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406299f0
//     owner=unresolved_namespace; count=6
//     target_aliases=28 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10879210","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386050192,"raw_hex":"e816ff59ff","size":5,"soft_fail":false,"target_va":5375171056,"va":5386050261,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141089a90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-633","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386050192,"raw_hex":"e887fdffff","size":5,"soft_fail":false,"target_va":5386050192,"va":5386050820,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17542935","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386050192,"raw_hex":"e817af0b01","size":5,"soft_fail":false,"target_va":5403593360,"va":5386050420,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirBuilder::_create
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-8641539","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386050192,"raw_hex":"e8fd237cff","size":5,"soft_fail":false,"target_va":5377408752,"va":5386050286,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirEntry::file_name
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-8642027","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386050192,"raw_hex":"e815227cff","size":5,"soft_fail":false,"target_va":5377408720,"va":5386050742,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirEntry::file_type
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-8641947","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386050192,"raw_hex":"e865227cff","size":5,"soft_fail":false,"target_va":5377408688,"va":5386050630,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirEntry::path
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-8642480","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386050192,"raw_hex":"e850207cff","size":5,"soft_fail":false,"target_va":5377408224,"va":5386050699,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-8611001","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386050192,"raw_hex":"e8479b7cff","size":5,"soft_fail":false,"target_va":5377439776,"va":5386050772,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::pal::windows::fs::readdir
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-8582140","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386050192,"raw_hex":"e8040c7dff","size":5,"soft_fail":false,"target_va":5377468224,"va":5386050359,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1431,"record_offset":122420,"parent_record_offset":119092,"ancestor_record_offset":119092,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1431,"record_offset":120460,"parent_record_offset":119092,"ancestor_record_offset":119092,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] std::fs::FileType::is_dir
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":1431,"record_offset":120560,"parent_record_offset":119092,"ancestor_record_offset":119092,"inlinee":8480,"source_line":1597,"archive":"_pdb/A/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] std::fs::create_dir_all
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":1431,"record_offset":119292,"parent_record_offset":119092,"ancestor_record_offset":119092,"inlinee":4768,"source_line":2306,"archive":"_pdb/A/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_dir
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":1431,"record_offset":119572,"parent_record_offset":119092,"ancestor_record_offset":119092,"inlinee":8478,"source_line":2465,"archive":"_pdb/A/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":1431,"record_offset":120812,"parent_record_offset":119092,"ancestor_record_offset":119092,"inlinee":4747,"source_line":2551,"archive":"_pdb/A/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$79::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":1431,"record_offset":120644,"parent_record_offset":119092,"ancestor_record_offset":119092,"inlinee":4752,"source_line":3182,"archive":"_pdb/A/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1431,"record_offset":120096,"parent_record_offset":120072,"ancestor_record_offset":119092,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1431,"record_offset":120072,"parent_record_offset":120008,"ancestor_record_offset":119092,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1431,"record_offset":120040,"parent_record_offset":120008,"ancestor_record_offset":119092,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1431,"record_offset":120008,"parent_record_offset":119976,"ancestor_record_offset":119092,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1431,"record_offset":122528,"parent_record_offset":122492,"ancestor_record_offset":119092,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1431,"record_offset":122492,"parent_record_offset":122468,"ancestor_record_offset":119092,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":1431,"record_offset":119344,"parent_record_offset":119292,"ancestor_record_offset":119092,"inlinee":4750,"source_line":701,"archive":"_pdb/A/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=77
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1431,"record_offset":119824,"parent_record_offset":119572,"ancestor_record_offset":119092,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1431,"record_offset":122560,"parent_record_offset":122528,"ancestor_record_offset":119092,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1431,"record_offset":119784,"parent_record_offset":119572,"ancestor_record_offset":119092,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1431,"record_offset":122588,"parent_record_offset":122528,"ancestor_record_offset":119092,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1431,"record_offset":122620,"parent_record_offset":122492,"ancestor_record_offset":119092,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":1431,"record_offset":119440,"parent_record_offset":119416,"ancestor_record_offset":119092,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":1431,"record_offset":119488,"parent_record_offset":119464,"ancestor_record_offset":119092,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":1431,"record_offset":119464,"parent_record_offset":119440,"ancestor_record_offset":119092,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::create
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":1431,"record_offset":119540,"parent_record_offset":119292,"ancestor_record_offset":119092,"inlinee":4771,"source_line":2561,"archive":"_pdb/A/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::recursive
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":1431,"record_offset":119316,"parent_record_offset":119292,"ancestor_record_offset":119092,"inlinee":4770,"source_line":2537,"archive":"_pdb/A/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":1431,"record_offset":119416,"parent_record_offset":119392,"ancestor_record_offset":119092,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$44::deref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":1431,"record_offset":119392,"parent_record_offset":119368,"ancestor_record_offset":119092,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$76::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":1431,"record_offset":120844,"parent_record_offset":120812,"ancestor_record_offset":119092,"inlinee":8485,"source_line":3158,"archive":"_pdb/A/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$79::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":1431,"record_offset":119368,"parent_record_offset":119344,"ancestor_record_offset":119092,"inlinee":4752,"source_line":3182,"archive":"_pdb/A/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::FileType::is_dir
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":1431,"record_offset":120584,"parent_record_offset":120560,"ancestor_record_offset":119092,"inlinee":8482,"source_line":1026,"archive":"_pdb/A/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::FileType::is_symlink
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":1431,"record_offset":120608,"parent_record_offset":120584,"ancestor_record_offset":119092,"inlinee":8483,"source_line":1032,"archive":"_pdb/A/modules/01431.jsonl.gz"}
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
// 完整结构及原始类型记录: startup_session.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::startup::startup_session::MaybeExists
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 0
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::startup::startup_session::SessionLoadAttempt
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 6
// [PDB 泛型关系，非字段关系] {"type": "enum2$<country_core::startup::startup_session::SessionLoadAttempt>", "constructor": "enum2$", "arguments": ["country_core::startup::startup_session::SessionLoadAttempt"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::collections::vec_deque::VecDeque<enum2$<country_core::startup::startup_session::SessionLoadAttempt>,alloc::alloc::Global>", "constructor": "alloc::collections::vec_deque::VecDeque", "arguments": ["enum2$<country_core::startup::startup_session::SessionLoadAttempt>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::collections::vec_deque::VecDeque<enum2$<country_core::startup::startup_session::SessionLoadAttempt>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::collections::vec_deque::VecDeque<enum2$<country_core::startup::startup_session::SessionLoadAttempt>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::reserve_for_push<enum2$<country_core::startup::startup_session::SessionLoadAttempt>,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::reserve_for_push", "arguments": ["enum2$<country_core::startup::startup_session::SessionLoadAttempt>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<enum2$<country_core::startup::startup_session::SessionLoadAttempt> >", "constructor": "core::option::Option", "arguments": ["enum2$<country_core::startup::startup_session::SessionLoadAttempt>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<enum2$<country_core::startup::startup_session::SessionLoadAttempt> > >", "constructor": "enum2$", "arguments": ["core::option::Option<enum2$<country_core::startup::startup_session::SessionLoadAttempt> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::option::Option<enum2$<country_core::startup::startup_session::SessionLoadAttempt> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::option::Option<enum2$<country_core::startup::startup_session::SessionLoadAttempt> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<country_core::startup::startup_session::SessionLoadAttempt> >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<country_core::startup::startup_session::SessionLoadAttempt>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::collections::vec_deque::VecDeque::grow<enum2$<country_core::startup::startup_session::SessionLoadAttempt>,alloc::alloc::Global>", "constructor": "alloc::collections::vec_deque::VecDeque::grow", "arguments": ["enum2$<country_core::startup::startup_session::SessionLoadAttempt>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: startup_session.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: startup_session.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 6; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>,bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >,bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::Toolbar>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>", "bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >", "bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>", "bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::Toolbar>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::Toolbar>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::Toolbar>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::Res<country_core::systems::history::history_process::PendingHistoryCommands>,bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::history::replay_mode::RestoreAppTo>,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::event::EventReader<country_core::systems::history::history_process::HistoryCmd>", "bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::Toolbar>", "bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>", "bevy_ecs::change_detection::Res<country_core::systems::history::history_process::PendingHistoryCommands>", "bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>", "bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "bevy_ecs::change_detection::Res<country_core::systems::history::replay_mode::RestoreAppTo>", "bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::history::history_process::HistoryCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::Toolbar>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::history::history_process::PendingHistoryCommands>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::history::replay_mode::RestoreAppTo>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession> > >,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::session::NextSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::event::EventWriter<country_core::resources::session::NewSessionStartedCmd>,bevy_ecs::event::EventWriter<country_core::resources::session::SessionLoadedCmd>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>", "enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession> > >", "bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>", "bevy_ecs::change_detection::ResMut<country_core::resources::session::NextSession>", "bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>", "bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>", "bevy_ecs::event::EventWriter<country_core::resources::session::NewSessionStartedCmd>", "bevy_ecs::event::EventWriter<country_core::resources::session::SessionLoadedCmd>", "bevy_ecs::system::commands::Commands", "bevy_ecs::event::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::session::NextSession>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::session::NewSessionStartedCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::session::SessionLoadedCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)
// 候选参数类型: ["country_core::systems::history::replay_raster::RasterReplayResources", "bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>", "bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>", "bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>", "bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>"]
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>
// [文件级接口候选，具体函数归属 unknown] bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >)
// 候选参数类型: ["enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::NextSession>

// ------------------------------------------------------------------
// 函数: country_core::startup::startup_session::MaybeExists::get
// 信息 ID: ba093de3ad6491c6c22df4c9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::startup_session::load_game_session; module=1330, inline_record=38060; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::startup_session::SessionLoadAttempt::from_session_id
// 信息 ID: 518ca3cdefb09bd0b87e7d2c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::startup_session::load_game_session; module=1330, inline_record=35064; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::startup_session::copy_dir_all<std::path::PathBuf,std::path::PathBuf>
// 信息 ID: b5d9e5e4891ba3c4fc0f4ea8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::startup::startup_session::copy_dir_all -> ["std::path::PathBuf", "std::path::PathBuf"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::startup_session::load_game_session @ 0xe96be2 -> 0x1089a90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_session::copy_dir_all<std::path::PathBuf,std::path::PathBuf> @ 0x1089d04 -> 0x1089a90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::startup_session::load_game_session
// 信息 ID: 7a93383fd0639d1a83692328
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] fe71f73f6916b0dec0c4e255: void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession> > >,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::session::NextSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::event::EventWriter<country_core::resources::session::NewSessionStartedCmd>,bevy_ecs::event::EventWriter<country_core::resources::session::SessionLoadedCmd>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession> > >,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::session::NextSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::event::EventWriter<country_core::resources::session::NewSessionStartedCmd>,bevy_ecs::event::EventWriter<country_core::resources::session::SessionLoadedCmd>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession> > >,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::session::NextSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::event::EventWriter<country_core::resources::session::NewSessionStartedCmd>,bevy_ecs::event::EventWriter<country_core::resources::session::SessionLoadedCmd>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)> > @ 0xe6d39f -> 0xe95060 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession> > >,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::session::NextSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::event::EventWriter<country_core::resources::session::NewSessionStartedCmd>,bevy_ecs::event::EventWriter<country_core::resources::session::SessionLoadedCmd>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession> > >,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::session::NextSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::event::EventWriter<country_core::resources::session::NewSessionStartedCmd>,bevy_ecs::event::EventWriter<country_core::resources::session::SessionLoadedCmd>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)> @ 0x1088f78 -> 0xe95060 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::startup_session::load_game_session::closure$0
// 信息 ID: b6f34a8c192f9ec916b56885
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::startup::startup_session::load_game_session", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::startup_session::load_game_session; module=1330, inline_record=49708; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::startup_session::load_game_session::closure$1
// 信息 ID: 0da89ec6980e88385c6a9897
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::startup::startup_session::load_game_session", "captures": "unknown"}
// [闭包环境类型引用] country_core::startup::startup_session::load_game_session::closure_env$1 <- anyhow::context::impl$0::with_context<std::fs::ReadDir,std::io::error::Error,alloc::string::String,country_core::startup::startup_session::load_game_session::closure_env$1>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] anyhow::context::impl$0::with_context<std::fs::ReadDir,std::io::error::Error,alloc::string::String,country_core::startup::startup_session::load_game_session::closure_env$1>; module=387, inline_record=3424; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::startup_session::load_game_session::closure$2
// 信息 ID: 9822a87b253919d25388dddb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "country_core::startup::startup_session::load_game_session", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::startup_session::load_game_session; module=1330, inline_record=44964; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::startup_session::load_session_meta
// 信息 ID: 65626edde4c4e1159fcdd1b3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::startup_session::load_game_session; module=1330, inline_record=48108; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
