// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/tiny-glade/src/gui/wall_selection_tags.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: c91cff839a05ca517ff2ee8d0e5c29a0296f8921da16e014183d756c3081495d
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: wall_selection_tags.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 67, 196, 222, 246
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use lang_lib; // namespace candidate
//   package identities: [{"name":"lang-lib","source":"local_or_path","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// use tg_text; // namespace candidate
//   package identities: [{"name":"tg-text","source":"local_or_path","version":"0.1.0"}]
// use tiny_glade; // namespace candidate
//   package identities: [{"name":"tiny-glade","source":"local_or_path","version":"0.2.0"}]
// use toolbar; // namespace candidate
//   package identities: [{"name":"toolbar","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::wall_selection_tags::wall_selection_tags
// 记录形式: compiled_procedure; instances=1
// 源码行: 30
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":246,"record_offset":12916,"rva":6758256,"va":5375467376}
// 编译实例 RVAs: 0x671f70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t211009","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375467376,"raw_hex":"e841380300","size":5,"soft_fail":false,"target_va":5375678688,"va":5375467674,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=8
//     evidence={"assembly":"\tcallq\t207893","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375467376,"raw_hex":"e8152c0300","size":5,"soft_fail":false,"target_va":5375678704,"va":5375470806,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: wall_selection_tags.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6745472","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375467376,"raw_hex":"e8801299ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5375467659,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400cb570
//     owner=unresolved_namespace; count=1
//     target_aliases=17 candidates; complete list: wall_selection_tags.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5927747","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375467376,"raw_hex":"e8bd8ca5ff","size":5,"soft_fail":false,"target_va":5369542000,"va":5375469742,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140222f20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_selection_tags.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4520858","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375467376,"raw_hex":"e86604bbff","size":5,"soft_fail":false,"target_va":5370949408,"va":5375470261,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140223a20
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: wall_selection_tags.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4517751","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375467376,"raw_hex":"e88910bbff","size":5,"soft_fail":false,"target_va":5370952224,"va":5375469970,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048dd60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_selection_tags.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1984591","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375467376,"raw_hex":"e8b1b7e1ff","size":5,"soft_fail":false,"target_va":5373484384,"va":5375468970,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048def0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_selection_tags.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1984225","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375467376,"raw_hex":"e81fb9e1ff","size":5,"soft_fail":false,"target_va":5373484784,"va":5375469004,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140557760
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_selection_tags.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1160154","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375467376,"raw_hex":"e8264ceeff","size":5,"soft_fail":false,"target_va":5374310240,"va":5375470389,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140557bc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_selection_tags.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1158155","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375467376,"raw_hex":"e8f553eeff","size":5,"soft_fail":false,"target_va":5374311360,"va":5375469510,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140698b50
//     owner=unresolved_namespace; count=4
//     target_aliases=10 candidates; complete list: wall_selection_tags.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t158010","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375467376,"raw_hex":"e83a690200","size":5,"soft_fail":false,"target_va":5375626064,"va":5375468049,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406c6ac0
//     owner=unresolved_namespace; count=1
//     target_aliases=15 candidates; complete list: wall_selection_tags.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t343593","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375467376,"raw_hex":"e8293e0500","size":5,"soft_fail":false,"target_va":5375814336,"va":5375470738,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14071fc50
//     owner=unresolved_namespace; count=1
//     target_aliases=10 candidates; complete list: wall_selection_tags.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t708613","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375467376,"raw_hex":"e805d00a00","size":5,"soft_fail":false,"target_va":5376179280,"va":5375470662,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14071fdc0
//     owner=unresolved_namespace; count=2
//     target_aliases=10 candidates; complete list: wall_selection_tags.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t709664","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375467376,"raw_hex":"e820d40a00","size":5,"soft_fail":false,"target_va":5376179648,"va":5375469979,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140725200
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: wall_selection_tags.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t732483","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375467376,"raw_hex":"e8432d0b00","size":5,"soft_fail":false,"target_va":5376201216,"va":5375468728,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a13df0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: wall_selection_tags.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3808369","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375467376,"raw_hex":"e8711c3a00","size":5,"soft_fail":false,"target_va":5379276272,"va":5375467898,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f400
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_selection_tags.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t40881117","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375467376,"raw_hex":"e8ddcb6f02","size":5,"soft_fail":false,"target_va":5416350720,"va":5375469598,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f4e0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: wall_selection_tags.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t40883086","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375467376,"raw_hex":"e88ed36f02","size":5,"soft_fail":false,"target_va":5416350944,"va":5375467853,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f550
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_selection_tags.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t40880604","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375467376,"raw_hex":"e8dcc96f02","size":5,"soft_fail":false,"target_va":5416351056,"va":5375470447,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f6a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: wall_selection_tags.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t40882590","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375467376,"raw_hex":"e89ed16f02","size":5,"soft_fail":false,"target_va":5416351392,"va":5375468797,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: wall_selection_tags.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t41585871","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375467376,"raw_hex":"e8cf8c7a02","size":5,"soft_fail":false,"target_va":5417057056,"va":5375471180,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bef0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: wall_selection_tags.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t41588300","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375467376,"raw_hex":"e84c967a02","size":5,"soft_fail":false,"target_va":5417058032,"va":5375469727,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t41149539","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375467376,"raw_hex":"e863e47302","size":5,"soft_fail":false,"target_va":5416620769,"va":5375471225,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t9324739","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375467376,"raw_hex":"e8c3488e00","size":5,"soft_fail":false,"target_va":5384794864,"va":5375470120,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t41149872","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375467376,"raw_hex":"e8b0e57302","size":5,"soft_fail":false,"target_va":5416620794,"va":5375470917,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t41150388","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375467376,"raw_hex":"e8b4e77302","size":5,"soft_fail":false,"target_va":5416621600,"va":5375471207,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::inner_walls::InnerWalls::contains
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5842461","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375467376,"raw_hex":"e81d265900","size":5,"soft_fail":false,"target_va":5381310688,"va":5375468222,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::mode_manager::WallSelectionGroups::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5069262","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375467376,"raw_hex":"e8ce594d00","size":5,"soft_fail":false,"target_va":5380537072,"va":5375467805,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::mode_manager::WallSelectionGroups::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5069014","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375467376,"raw_hex":"e8d6584d00","size":5,"soft_fail":false,"target_va":5380537040,"va":5375468021,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::mode_manager::WallSelectionGroups::remove
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5066574","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375467376,"raw_hex":"e84e4f4d00","size":5,"soft_fail":false,"target_va":5380537584,"va":5375471005,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::mode_manager::WallSelectionGroups::save
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5067657","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375467376,"raw_hex":"e889534d00","size":5,"soft_fail":false,"target_va":5380537216,"va":5375469554,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7665280","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375467376,"raw_hex":"e880f67400","size":5,"soft_fail":false,"target_va":5383134720,"va":5375469435,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::MeiContext::reuse_or_create_node
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t737600","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375467376,"raw_hex":"e840410b00","size":5,"soft_fail":false,"target_va":5376206368,"va":5375468763,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::Node::ignore_input
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t763047","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375467376,"raw_hex":"e8a7a40b00","size":5,"soft_fail":false,"target_va":5376233040,"va":5375469988,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::Node::used
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t762444","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375467376,"raw_hex":"e84ca20b00","size":5,"soft_fail":false,"target_va":5376233120,"va":5375470671,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::child_with_global_id
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t739332","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375467376,"raw_hex":"e804480b00","size":5,"soft_fail":false,"target_va":5376208368,"va":5375469031,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::layout
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t740654","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375467376,"raw_hex":"e82e4d0b00","size":5,"soft_fail":false,"target_va":5376209536,"va":5375468877,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::layout::LayoutStyle::align_center
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t784923","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375467376,"raw_hex":"e81bfa0b00","size":5,"soft_fail":false,"target_va":5376253568,"va":5375468640,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::layout::LayoutStyle::align_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t785584","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375467376,"raw_hex":"e8b0fc0b00","size":5,"soft_fail":false,"target_va":5376254032,"va":5375468443,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::layout::LayoutStyle::gap
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t785158","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375467376,"raw_hex":"e806fb0b00","size":5,"soft_fail":false,"target_va":5376253776,"va":5375468613,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::layout::LayoutStyle::padding
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t785408","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375467376,"raw_hex":"e800fc0b00","size":5,"soft_fail":false,"target_va":5376253968,"va":5375468555,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::layout::LayoutStyle::xalign_center
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t785253","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375467376,"raw_hex":"e865fb0b00","size":5,"soft_fail":false,"target_va":5376253728,"va":5375468470,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::layout::horizontal
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t785099","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375467376,"raw_hex":"e8cbfa0b00","size":5,"soft_fail":false,"target_va":5376253520,"va":5375468416,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::layout::vertical
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t785532","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375467376,"raw_hex":"e87cfc0b00","size":5,"soft_fail":false,"target_va":5376254112,"va":5375468575,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::text_hint_popup
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t711972","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375467376,"raw_hex":"e824dd0a00","size":5,"soft_fail":false,"target_va":5376182752,"va":5375470775,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":246,"record_offset":13152,"parent_record_offset":12916,"ancestor_record_offset":12916,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":246,"record_offset":16324,"parent_record_offset":12916,"ancestor_record_offset":12916,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":246,"record_offset":17544,"parent_record_offset":12916,"ancestor_record_offset":12916,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":246,"record_offset":14840,"parent_record_offset":12916,"ancestor_record_offset":12916,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":246,"record_offset":13392,"parent_record_offset":12916,"ancestor_record_offset":12916,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":246,"record_offset":13480,"parent_record_offset":12916,"ancestor_record_offset":12916,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":246,"record_offset":15332,"parent_record_offset":12916,"ancestor_record_offset":12916,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":246,"record_offset":15388,"parent_record_offset":12916,"ancestor_record_offset":12916,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":246,"record_offset":14868,"parent_record_offset":12916,"ancestor_record_offset":12916,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_writer::MessageWriter::write
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":246,"record_offset":15636,"parent_record_offset":12916,"ancestor_record_offset":12916,"inlinee":11206,"source_line":69,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":246,"record_offset":16408,"parent_record_offset":12916,"ancestor_record_offset":12916,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::copied::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":246,"record_offset":13336,"parent_record_offset":12916,"ancestor_record_offset":12916,"inlinee":7432,"source_line":51,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":246,"record_offset":13696,"parent_record_offset":12916,"ancestor_record_offset":12916,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":246,"record_offset":14412,"parent_record_offset":12916,"ancestor_record_offset":12916,"inlinee":12047,"source_line":2863,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":246,"record_offset":13420,"parent_record_offset":12916,"ancestor_record_offset":12916,"inlinee":4877,"source_line":684,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":246,"record_offset":15604,"parent_record_offset":12916,"ancestor_record_offset":12916,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":246,"record_offset":14012,"parent_record_offset":12916,"ancestor_record_offset":12916,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":246,"record_offset":17260,"parent_record_offset":12916,"ancestor_record_offset":12916,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":246,"record_offset":14048,"parent_record_offset":12916,"ancestor_record_offset":12916,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_direct] core::time::Duration::as_secs_f32
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\time.rs
//     evidence={"module":246,"record_offset":15980,"parent_record_offset":12916,"ancestor_record_offset":12916,"inlinee":7360,"source_line":885,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::sin
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":246,"record_offset":16008,"parent_record_offset":12916,"ancestor_record_offset":12916,"inlinee":11242,"source_line":699,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::MeiContext::fullscreen_without_block_input
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":246,"record_offset":14928,"parent_record_offset":12916,"ancestor_record_offset":12916,"inlinee":10271,"source_line":471,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::MeiContext::new_relative
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":246,"record_offset":16036,"parent_record_offset":12916,"ancestor_record_offset":12916,"inlinee":10269,"source_line":505,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":246,"record_offset":15216,"parent_record_offset":12916,"ancestor_record_offset":12916,"inlinee":8058,"source_line":1443,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":246,"record_offset":13236,"parent_record_offset":13212,"ancestor_record_offset":12916,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":246,"record_offset":13260,"parent_record_offset":13236,"ancestor_record_offset":12916,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":246,"record_offset":17436,"parent_record_offset":17412,"ancestor_record_offset":12916,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":246,"record_offset":17460,"parent_record_offset":17436,"ancestor_record_offset":12916,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":246,"record_offset":13284,"parent_record_offset":13260,"ancestor_record_offset":12916,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":246,"record_offset":17484,"parent_record_offset":17460,"ancestor_record_offset":12916,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":246,"record_offset":13212,"parent_record_offset":13180,"ancestor_record_offset":12916,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":246,"record_offset":17412,"parent_record_offset":17356,"ancestor_record_offset":12916,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":246,"record_offset":13180,"parent_record_offset":13152,"ancestor_record_offset":12916,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":246,"record_offset":16372,"parent_record_offset":16348,"ancestor_record_offset":12916,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":246,"record_offset":13572,"parent_record_offset":13540,"ancestor_record_offset":12916,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":246,"record_offset":17664,"parent_record_offset":17636,"ancestor_record_offset":12916,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":246,"record_offset":17384,"parent_record_offset":17356,"ancestor_record_offset":12916,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":246,"record_offset":17356,"parent_record_offset":17332,"ancestor_record_offset":12916,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":246,"record_offset":13620,"parent_record_offset":13596,"ancestor_record_offset":12916,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":246,"record_offset":13596,"parent_record_offset":13572,"ancestor_record_offset":12916,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":246,"record_offset":17728,"parent_record_offset":17692,"ancestor_record_offset":12916,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":246,"record_offset":17692,"parent_record_offset":17664,"ancestor_record_offset":12916,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":246,"record_offset":17332,"parent_record_offset":17308,"ancestor_record_offset":12916,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":246,"record_offset":17768,"parent_record_offset":17728,"ancestor_record_offset":12916,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":246,"record_offset":17572,"parent_record_offset":17544,"ancestor_record_offset":12916,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":246,"record_offset":17600,"parent_record_offset":17572,"ancestor_record_offset":12916,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":246,"record_offset":13540,"parent_record_offset":13504,"ancestor_record_offset":12916,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":246,"record_offset":14992,"parent_record_offset":14952,"ancestor_record_offset":12916,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":246,"record_offset":13504,"parent_record_offset":13480,"ancestor_record_offset":12916,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":246,"record_offset":17636,"parent_record_offset":17600,"ancestor_record_offset":12916,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":246,"record_offset":18132,"parent_record_offset":18108,"ancestor_record_offset":12916,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":246,"record_offset":18180,"parent_record_offset":18156,"ancestor_record_offset":12916,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":246,"record_offset":14896,"parent_record_offset":14868,"ancestor_record_offset":12916,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":246,"record_offset":15660,"parent_record_offset":15636,"ancestor_record_offset":12916,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":246,"record_offset":15720,"parent_record_offset":15660,"ancestor_record_offset":12916,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":246,"record_offset":17792,"parent_record_offset":17768,"ancestor_record_offset":12916,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":246,"record_offset":13752,"parent_record_offset":13724,"ancestor_record_offset":12916,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":246,"record_offset":17872,"parent_record_offset":17816,"ancestor_record_offset":12916,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::copy_try_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":246,"record_offset":14544,"parent_record_offset":14460,"ancestor_record_offset":12916,"inlinee":8671,"source_line":40,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":246,"record_offset":14436,"parent_record_offset":14412,"ancestor_record_offset":12916,"inlinee":8669,"source_line":68,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":246,"record_offset":13724,"parent_record_offset":13696,"ancestor_record_offset":12916,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::any::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":246,"record_offset":14568,"parent_record_offset":14544,"ancestor_record_offset":12916,"inlinee":12049,"source_line":2870,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":246,"record_offset":14460,"parent_record_offset":14436,"ancestor_record_offset":12916,"inlinee":5047,"source_line":2461,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":246,"record_offset":17816,"parent_record_offset":17792,"ancestor_record_offset":12916,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":246,"record_offset":17844,"parent_record_offset":17816,"ancestor_record_offset":12916,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":246,"record_offset":18308,"parent_record_offset":18280,"ancestor_record_offset":12916,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":246,"record_offset":13360,"parent_record_offset":13336,"ancestor_record_offset":12916,"inlinee":7434,"source_line":2119,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":246,"record_offset":13444,"parent_record_offset":13420,"ancestor_record_offset":12916,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":246,"record_offset":16348,"parent_record_offset":16324,"ancestor_record_offset":12916,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":246,"record_offset":18040,"parent_record_offset":17600,"ancestor_record_offset":12916,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":246,"record_offset":18064,"parent_record_offset":18040,"ancestor_record_offset":12916,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":246,"record_offset":17284,"parent_record_offset":17260,"ancestor_record_offset":12916,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":246,"record_offset":15356,"parent_record_offset":15332,"ancestor_record_offset":12916,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":246,"record_offset":14148,"parent_record_offset":14080,"ancestor_record_offset":12916,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":246,"record_offset":15480,"parent_record_offset":15388,"ancestor_record_offset":12916,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":246,"record_offset":14116,"parent_record_offset":14080,"ancestor_record_offset":12916,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":246,"record_offset":15508,"parent_record_offset":15480,"ancestor_record_offset":12916,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":246,"record_offset":13660,"parent_record_offset":13504,"ancestor_record_offset":12916,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$17::slice_contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":246,"record_offset":14228,"parent_record_offset":14204,"ancestor_record_offset":12916,"inlinee":12688,"source_line":375,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$17::slice_contains::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":246,"record_offset":14336,"parent_record_offset":14252,"ancestor_record_offset":12916,"inlinee":12690,"source_line":376,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":246,"record_offset":14204,"parent_record_offset":14180,"ancestor_record_offset":12916,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":246,"record_offset":14252,"parent_record_offset":14228,"ancestor_record_offset":12916,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":246,"record_offset":14080,"parent_record_offset":14048,"ancestor_record_offset":12916,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$10::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":246,"record_offset":14360,"parent_record_offset":14336,"ancestor_record_offset":12916,"inlinee":12117,"source_line":82,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::gui::wall_selection_tags::wall_selection_tags::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/wall_selection_tags.rs
//     evidence={"module":246,"record_offset":14180,"parent_record_offset":14048,"ancestor_record_offset":12916,"inlinee":17750,"source_line":66,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::gui::wall_selection_tags::wall_selection_tags::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/wall_selection_tags.rs
//     evidence={"module":246,"record_offset":14592,"parent_record_offset":14568,"ancestor_record_offset":12916,"inlinee":16844,"source_line":74,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::MeiContext::new_root
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":246,"record_offset":14952,"parent_record_offset":14928,"ancestor_record_offset":12916,"inlinee":10221,"source_line":526,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":246,"record_offset":15264,"parent_record_offset":15240,"ancestor_record_offset":12916,"inlinee":8062,"source_line":115,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":246,"record_offset":15296,"parent_record_offset":15240,"ancestor_record_offset":12916,"inlinee":8064,"source_line":148,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$9::to_global_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":246,"record_offset":15240,"parent_record_offset":15216,"ancestor_record_offset":12916,"inlinee":8060,"source_line":154,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::wall_selection_tags::wall_selection_tags::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 66
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4556
// 首条位置: {"module":246,"inlinee":17750}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":246,"record_offset":14204,"parent_record_offset":14180,"ancestor_record_offset":14180,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":246,"record_offset":14304,"parent_record_offset":14280,"ancestor_record_offset":14180,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$17::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":246,"record_offset":14228,"parent_record_offset":14204,"ancestor_record_offset":14180,"inlinee":12688,"source_line":375,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$17::slice_contains::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":246,"record_offset":14336,"parent_record_offset":14252,"ancestor_record_offset":14180,"inlinee":12690,"source_line":376,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":246,"record_offset":14252,"parent_record_offset":14228,"ancestor_record_offset":14180,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":246,"record_offset":14280,"parent_record_offset":14252,"ancestor_record_offset":14180,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$10::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":246,"record_offset":14360,"parent_record_offset":14336,"ancestor_record_offset":14180,"inlinee":12117,"source_line":82,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::wall_selection_tags::wall_selection_tags::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 74
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x41CC
// 首条位置: {"module":222,"inlinee":16844}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":246,"record_offset":14616,"parent_record_offset":14592,"ancestor_record_offset":14592,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":246,"record_offset":14716,"parent_record_offset":14692,"ancestor_record_offset":14592,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$17::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":246,"record_offset":14640,"parent_record_offset":14616,"ancestor_record_offset":14592,"inlinee":12688,"source_line":375,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$17::slice_contains::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":246,"record_offset":14748,"parent_record_offset":14664,"ancestor_record_offset":14592,"inlinee":12690,"source_line":376,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":246,"record_offset":14664,"parent_record_offset":14640,"ancestor_record_offset":14592,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":246,"record_offset":14692,"parent_record_offset":14664,"ancestor_record_offset":14592,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$10::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":246,"record_offset":14772,"parent_record_offset":14748,"ancestor_record_offset":14592,"inlinee":12117,"source_line":82,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::wall_selection_tags::impl$2::default
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 192
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x27F9
// 首条位置: {"module":67,"inlinee":10233}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::wall_selection_tags::impl$0::create_node_content
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 201
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x27F7
// 首条位置: {"module":67,"inlinee":10231}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":67,"record_offset":11332,"parent_record_offset":11276,"ancestor_record_offset":11276,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00067.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::gui::wall_selection_tags::impl$2::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/wall_selection_tags.rs
//     evidence={"module":67,"record_offset":11304,"parent_record_offset":11276,"ancestor_record_offset":11276,"inlinee":10233,"source_line":192,"archive":"_pdb/C/modules/00067.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":67,"record_offset":11380,"parent_record_offset":11356,"ancestor_record_offset":11276,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00067.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":67,"record_offset":11356,"parent_record_offset":11332,"ancestor_record_offset":11276,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00067.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":67,"record_offset":11416,"parent_record_offset":11380,"ancestor_record_offset":11276,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00067.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":67,"record_offset":11440,"parent_record_offset":11416,"ancestor_record_offset":11276,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00067.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":67,"record_offset":11472,"parent_record_offset":11380,"ancestor_record_offset":11276,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00067.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::wall_selection_tags::impl$1::prepare_layout
// 记录形式: compiled_procedure; instances=1
// 源码行: 207
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":246,"record_offset":248,"rva":6750240,"va":5375459360}
// 编译实例 RVAs: 0x670020
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t218942","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375459360,"raw_hex":"e83e570300","size":5,"soft_fail":false,"target_va":5375678704,"va":5375459757,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400a5950
//     owner=unresolved_namespace; count=4
//     target_aliases=4 candidates; complete list: wall_selection_tags.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6072885","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375459360,"raw_hex":"e8cb55a3ff","size":5,"soft_fail":false,"target_va":5369387344,"va":5375460224,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140698b40
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: wall_selection_tags.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t166594","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375459360,"raw_hex":"e8c28a0200","size":5,"soft_fail":false,"target_va":5375626048,"va":5375459449,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140726440
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: wall_selection_tags.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t745813","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375459360,"raw_hex":"e855610b00","size":5,"soft_fail":false,"target_va":5376205888,"va":5375460070,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b8f4d0
//     owner=unresolved_namespace; count=2
//     target_aliases=20 candidates; complete list: wall_selection_tags.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5370950","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375459360,"raw_hex":"e846f45100","size":5,"soft_fail":false,"target_va":5380830416,"va":5375459461,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cb32b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: wall_selection_tags.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6566406","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375459360,"raw_hex":"e806326400","size":5,"soft_fail":false,"target_va":5382025904,"va":5375459493,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f2b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: wall_selection_tags.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t40890846","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375459360,"raw_hex":"e8def16f02","size":5,"soft_fail":false,"target_va":5416350384,"va":5375459533,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::shape_stream::RawShapeStream::append
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t6565693","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375459360,"raw_hex":"e83d2f6400","size":5,"soft_fail":false,"target_va":5382025312,"va":5375459614,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::shape_stream::ShapeStream::fill_color
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t6565270","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375459360,"raw_hex":"e8962d6400","size":5,"soft_fail":false,"target_va":5382024752,"va":5375459477,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::num::impl$8::to_be_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":246,"record_offset":1044,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":13480,"source_line":3868,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":246,"record_offset":1128,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_direct] tg_text::shape_stream::ShapeStream::image
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-text/src/shape_stream.rs
//     evidence={"module":246,"record_offset":400,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":7180,"source_line":26,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":246,"record_offset":912,"parent_record_offset":888,"ancestor_record_offset":248,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":246,"record_offset":936,"parent_record_offset":912,"ancestor_record_offset":248,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":246,"record_offset":960,"parent_record_offset":936,"ancestor_record_offset":248,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":246,"record_offset":888,"parent_record_offset":864,"ancestor_record_offset":248,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":246,"record_offset":580,"parent_record_offset":548,"ancestor_record_offset":248,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":246,"record_offset":1424,"parent_record_offset":1396,"ancestor_record_offset":248,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":246,"record_offset":864,"parent_record_offset":840,"ancestor_record_offset":248,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":246,"record_offset":628,"parent_record_offset":604,"ancestor_record_offset":248,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":246,"record_offset":604,"parent_record_offset":580,"ancestor_record_offset":248,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":246,"record_offset":840,"parent_record_offset":816,"ancestor_record_offset":248,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":246,"record_offset":548,"parent_record_offset":504,"ancestor_record_offset":248,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":246,"record_offset":476,"parent_record_offset":400,"ancestor_record_offset":248,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":246,"record_offset":504,"parent_record_offset":476,"ancestor_record_offset":248,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":246,"record_offset":1176,"parent_record_offset":1152,"ancestor_record_offset":248,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":246,"record_offset":448,"parent_record_offset":400,"ancestor_record_offset":248,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":246,"record_offset":1708,"parent_record_offset":1680,"ancestor_record_offset":248,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::swap_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":246,"record_offset":1092,"parent_record_offset":1068,"ancestor_record_offset":248,"inlinee":13482,"source_line":559,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::to_be
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":246,"record_offset":1068,"parent_record_offset":1044,"ancestor_record_offset":248,"inlinee":13481,"source_line":701,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=30
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":246,"record_offset":696,"parent_record_offset":504,"ancestor_record_offset":248,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":246,"record_offset":668,"parent_record_offset":504,"ancestor_record_offset":248,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::wall_selection_tags::impl$1::render
// 记录形式: compiled_procedure; instances=1
// 源码行: 240
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":246,"record_offset":3264,"rva":6751408,"va":5375460528}
// 编译实例 RVAs: 0x6704b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] toolbar::mei::core::RenderContext::draw_rect
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t749480","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375460528,"raw_hex":"e8a86f0b00","size":5,"soft_fail":false,"target_va":5376210112,"va":5375460627,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":246,"record_offset":3408,"parent_record_offset":3264,"ancestor_record_offset":3264,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":246,"record_offset":3560,"parent_record_offset":3264,"ancestor_record_offset":3264,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":246,"record_offset":3484,"parent_record_offset":3264,"ancestor_record_offset":3264,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":246,"record_offset":3584,"parent_record_offset":3560,"ancestor_record_offset":3264,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":246,"record_offset":3432,"parent_record_offset":3408,"ancestor_record_offset":3264,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":246,"record_offset":3508,"parent_record_offset":3484,"ancestor_record_offset":3264,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::wall_selection_tags::gift_tag<ref$<str$> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 254
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":196,"record_offset":1224,"rva":5602240,"va":5374311360}
// 编译实例 RVAs: 0x557bc0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140222890
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_selection_tags.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3364187","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374311360,"raw_hex":"e8a5aaccff","size":5,"soft_fail":false,"target_va":5370947728,"va":5374311910,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140223510
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_selection_tags.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3361351","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374311360,"raw_hex":"e8b9b5ccff","size":5,"soft_fail":false,"target_va":5370950928,"va":5374312274,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048dd60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_selection_tags.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-827480","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374311360,"raw_hex":"e8a85ff3ff","size":5,"soft_fail":false,"target_va":5373484384,"va":5374311859,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048def0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_selection_tags.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-827104","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374311360,"raw_hex":"e82061f3ff","size":5,"soft_fail":false,"target_va":5373484784,"va":5374311883,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140607d00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_selection_tags.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t720273","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374311360,"raw_hex":"e891fd0a00","size":5,"soft_fail":false,"target_va":5375032576,"va":5374312298,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14071fdc0
//     owner=unresolved_namespace; count=2
//     target_aliases=10 candidates; complete list: wall_selection_tags.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1867449","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374311360,"raw_hex":"e8b97e1c00","size":5,"soft_fail":false,"target_va":5376179648,"va":5374312194,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140725200
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_selection_tags.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1889379","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374311360,"raw_hex":"e863d41c00","size":5,"soft_fail":false,"target_va":5376201216,"va":5374311832,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140727090
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: wall_selection_tags.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1896954","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374311360,"raw_hex":"e8faf11c00","size":5,"soft_fail":false,"target_va":5376209040,"va":5374312081,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072dc40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_selection_tags.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1924442","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374311360,"raw_hex":"e85a5d1d00","size":5,"soft_fail":false,"target_va":5376236608,"va":5374312161,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140731e10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_selection_tags.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1941309","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374311360,"raw_hex":"e83d9f1d00","size":5,"soft_fail":false,"target_va":5376253456,"va":5374312142,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1be30
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: wall_selection_tags.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42746313","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374311360,"raw_hex":"e8c9418c02","size":5,"soft_fail":false,"target_va":5417057840,"va":5374311522,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::Node::force_interactive
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1920865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374311360,"raw_hex":"e8614f1d00","size":5,"soft_fail":false,"target_va":5376233072,"va":5374312202,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::Node::ignore_input
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1920668","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374311360,"raw_hex":"e89c4e1d00","size":5,"soft_fail":false,"target_va":5376233040,"va":5374312367,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::child_with_global_id
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1896461","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374311360,"raw_hex":"e80df01c00","size":5,"soft_fail":false,"target_va":5376208368,"va":5374311902,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::hover
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1897070","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374311360,"raw_hex":"e86ef21c00","size":5,"soft_fail":false,"target_va":5376209296,"va":5374312221,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::layout
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1897465","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374311360,"raw_hex":"e8f9f31c00","size":5,"soft_fail":false,"target_va":5376209536,"va":5374312066,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::used
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1896820","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374311360,"raw_hex":"e874f11c00","size":5,"soft_fail":false,"target_va":5376209056,"va":5374312231,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::with_nav
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1897698","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374311360,"raw_hex":"e8e2f41c00","size":5,"soft_fail":false,"target_va":5376209840,"va":5374312137,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::layout::LayoutStyle::padding
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1941918","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374311360,"raw_hex":"e89ea11d00","size":5,"soft_fail":false,"target_va":5376253968,"va":5374312045,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::layout::LayoutStyle::size
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1941843","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374311360,"raw_hex":"e853a11d00","size":5,"soft_fail":false,"target_va":5376253840,"va":5374311992,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::layout::horizontal
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1941559","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374311360,"raw_hex":"e837a01d00","size":5,"soft_fail":false,"target_va":5376253520,"va":5374311956,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::LabelView::color
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1872876","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374311360,"raw_hex":"e8ec931c00","size":5,"soft_fail":false,"target_va":5376185232,"va":5374312351,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::LabelView::content_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1872865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374311360,"raw_hex":"e8e1931c00","size":5,"soft_fail":false,"target_va":5376185152,"va":5374312282,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::text_hint_popup
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1870329","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374311360,"raw_hex":"e8f9891c00","size":5,"soft_fail":false,"target_va":5376182752,"va":5374312418,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::radical_inverse
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8964106","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374311360,"raw_hex":"e80ac88800","size":5,"soft_fail":false,"target_va":5383275552,"va":5374311441,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":196,"record_offset":1388,"parent_record_offset":1224,"ancestor_record_offset":1224,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/00196.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$8::from_be_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":196,"record_offset":1500,"parent_record_offset":1224,"ancestor_record_offset":1224,"inlinee":15943,"source_line":3956,"archive":"_pdb/C/modules/00196.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::gui::wall_selection_tags::gift_tag::closure$0
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/wall_selection_tags.rs
//     evidence={"module":196,"record_offset":1416,"parent_record_offset":1224,"ancestor_record_offset":1224,"inlinee":15942,"source_line":281,"archive":"_pdb/C/modules/00196.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::child_with_content
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":196,"record_offset":1584,"parent_record_offset":1224,"ancestor_record_offset":1224,"inlinee":10209,"source_line":1427,"archive":"_pdb/C/modules/00196.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::layout::Size::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":196,"record_offset":1740,"parent_record_offset":1224,"ancestor_record_offset":1224,"inlinee":11447,"source_line":45,"archive":"_pdb/C/modules/00196.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::metrics
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":196,"record_offset":1796,"parent_record_offset":1224,"ancestor_record_offset":1224,"inlinee":9902,"source_line":273,"archive":"_pdb/C/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::from_be
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":196,"record_offset":1524,"parent_record_offset":1500,"ancestor_record_offset":1224,"inlinee":15944,"source_line":638,"archive":"_pdb/C/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::swap_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":196,"record_offset":1548,"parent_record_offset":1524,"ancestor_record_offset":1224,"inlinee":13482,"source_line":559,"archive":"_pdb/C/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::gui::wall_selection_tags::gift_tag::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/wall_selection_tags.rs
//     evidence={"module":196,"record_offset":1704,"parent_record_offset":1584,"ancestor_record_offset":1224,"inlinee":10229,"source_line":284,"archive":"_pdb/C/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":196,"record_offset":1648,"parent_record_offset":1584,"ancestor_record_offset":1224,"inlinee":8058,"source_line":1443,"archive":"_pdb/C/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":196,"record_offset":1616,"parent_record_offset":1584,"ancestor_record_offset":1224,"inlinee":8062,"source_line":115,"archive":"_pdb/C/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":196,"record_offset":1672,"parent_record_offset":1648,"ancestor_record_offset":1224,"inlinee":8064,"source_line":148,"archive":"_pdb/C/modules/00196.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::wall_selection_tags::gift_tag<u64>
// 记录形式: compiled_procedure; instances=1
// 源码行: 254
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":196,"record_offset":536,"rva":5601120,"va":5374310240}
// 编译实例 RVAs: 0x557760
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400cb570
//     owner=unresolved_namespace; count=1
//     target_aliases=17 candidates; complete list: wall_selection_tags.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4768701","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374310240,"raw_hex":"e8433cb7ff","size":5,"soft_fail":false,"target_va":5369542000,"va":5374310696,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140222890
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_selection_tags.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3363051","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374310240,"raw_hex":"e815afccff","size":5,"soft_fail":false,"target_va":5370947728,"va":5374310774,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140223510
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_selection_tags.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3360215","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374310240,"raw_hex":"e829baccff","size":5,"soft_fail":false,"target_va":5370950928,"va":5374311138,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048dd60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_selection_tags.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-826344","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374310240,"raw_hex":"e81864f3ff","size":5,"soft_fail":false,"target_va":5373484384,"va":5374310723,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048def0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_selection_tags.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-825968","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374310240,"raw_hex":"e89065f3ff","size":5,"soft_fail":false,"target_va":5373484784,"va":5374310747,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140607d00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_selection_tags.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t721409","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374310240,"raw_hex":"e801020b00","size":5,"soft_fail":false,"target_va":5375032576,"va":5374311162,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14071fdc0
//     owner=unresolved_namespace; count=2
//     target_aliases=10 candidates; complete list: wall_selection_tags.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1868585","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374310240,"raw_hex":"e829831c00","size":5,"soft_fail":false,"target_va":5376179648,"va":5374311058,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140727090
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: wall_selection_tags.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1898090","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374310240,"raw_hex":"e86af61c00","size":5,"soft_fail":false,"target_va":5376209040,"va":5374310945,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072dc40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_selection_tags.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1925578","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374310240,"raw_hex":"e8ca611d00","size":5,"soft_fail":false,"target_va":5376236608,"va":5374311025,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140731e10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_selection_tags.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1942445","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374310240,"raw_hex":"e8ada31d00","size":5,"soft_fail":false,"target_va":5376253456,"va":5374311006,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1be30
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: wall_selection_tags.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42747441","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374310240,"raw_hex":"e831468c02","size":5,"soft_fail":false,"target_va":5417057840,"va":5374310394,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::Node::force_interactive
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1922001","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374310240,"raw_hex":"e8d1531d00","size":5,"soft_fail":false,"target_va":5376233072,"va":5374311066,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::Node::ignore_input
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1921804","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374310240,"raw_hex":"e80c531d00","size":5,"soft_fail":false,"target_va":5376233040,"va":5374311231,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::child_with_global_id
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1897597","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374310240,"raw_hex":"e87df41c00","size":5,"soft_fail":false,"target_va":5376208368,"va":5374310766,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::hover
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1898206","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374310240,"raw_hex":"e8def61c00","size":5,"soft_fail":false,"target_va":5376209296,"va":5374311085,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::layout
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1898601","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374310240,"raw_hex":"e869f81c00","size":5,"soft_fail":false,"target_va":5376209536,"va":5374310930,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::used
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1897956","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374310240,"raw_hex":"e8e4f51c00","size":5,"soft_fail":false,"target_va":5376209056,"va":5374311095,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::with_nav
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1898834","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374310240,"raw_hex":"e852f91c00","size":5,"soft_fail":false,"target_va":5376209840,"va":5374311001,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::layout::LayoutStyle::padding
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1943054","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374310240,"raw_hex":"e80ea61d00","size":5,"soft_fail":false,"target_va":5376253968,"va":5374310909,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::layout::LayoutStyle::size
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1942979","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374310240,"raw_hex":"e8c3a51d00","size":5,"soft_fail":false,"target_va":5376253840,"va":5374310856,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::layout::horizontal
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1942695","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374310240,"raw_hex":"e8a7a41d00","size":5,"soft_fail":false,"target_va":5376253520,"va":5374310820,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::LabelView::color
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1874012","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374310240,"raw_hex":"e85c981c00","size":5,"soft_fail":false,"target_va":5376185232,"va":5374311215,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::LabelView::content_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1874001","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374310240,"raw_hex":"e851981c00","size":5,"soft_fail":false,"target_va":5376185152,"va":5374311146,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::text_hint_popup
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1871465","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374310240,"raw_hex":"e8698e1c00","size":5,"soft_fail":false,"target_va":5376182752,"va":5374311282,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::radical_inverse
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8965234","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374310240,"raw_hex":"e872cc8800","size":5,"soft_fail":false,"target_va":5383275552,"va":5374310313,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":196,"record_offset":692,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/00196.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$8::from_be_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":196,"record_offset":804,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":15943,"source_line":3956,"archive":"_pdb/C/modules/00196.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::gui::wall_selection_tags::gift_tag::closure$0
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/wall_selection_tags.rs
//     evidence={"module":196,"record_offset":720,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":15942,"source_line":281,"archive":"_pdb/C/modules/00196.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::child_with_content
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":196,"record_offset":888,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":10209,"source_line":1427,"archive":"_pdb/C/modules/00196.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::layout::Size::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":196,"record_offset":1044,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":11447,"source_line":45,"archive":"_pdb/C/modules/00196.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::metrics
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":196,"record_offset":1100,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":9902,"source_line":273,"archive":"_pdb/C/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::from_be
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":196,"record_offset":828,"parent_record_offset":804,"ancestor_record_offset":536,"inlinee":15944,"source_line":638,"archive":"_pdb/C/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::swap_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":196,"record_offset":852,"parent_record_offset":828,"ancestor_record_offset":536,"inlinee":13482,"source_line":559,"archive":"_pdb/C/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::gui::wall_selection_tags::gift_tag::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/wall_selection_tags.rs
//     evidence={"module":196,"record_offset":1008,"parent_record_offset":888,"ancestor_record_offset":536,"inlinee":10229,"source_line":284,"archive":"_pdb/C/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":196,"record_offset":952,"parent_record_offset":888,"ancestor_record_offset":536,"inlinee":8058,"source_line":1443,"archive":"_pdb/C/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":196,"record_offset":920,"parent_record_offset":888,"ancestor_record_offset":536,"inlinee":8062,"source_line":115,"archive":"_pdb/C/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":196,"record_offset":976,"parent_record_offset":952,"ancestor_record_offset":536,"inlinee":8064,"source_line":148,"archive":"_pdb/C/modules/00196.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::wall_selection_tags::gift_tag::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 281
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3E46
// 首条位置: {"module":196,"inlinee":15942}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::wall_selection_tags::gift_tag::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 284
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x27F5
// 首条位置: {"module":67,"inlinee":10229}
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
// 完整结构及原始类型记录: wall_selection_tags.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::gui::wall_selection_tags::GiftTagContent
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 11
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tiny_glade::gui::wall_selection_tags::GiftTagContent>", "constructor": "core::ptr::drop_in_place", "arguments": ["tiny_glade::gui::wall_selection_tags::GiftTagContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<tiny_glade::gui::wall_selection_tags::GiftTagContent>", "constructor": "core::any::impl$0::type_id", "arguments": ["tiny_glade::gui::wall_selection_tags::GiftTagContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::NodeContext::get_content_mut_impl<tiny_glade::gui::wall_selection_tags::GiftTagContent>", "constructor": "toolbar::mei::core::NodeContext::get_content_mut_impl", "arguments": ["tiny_glade::gui::wall_selection_tags::GiftTagContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::impl$21::as_any_mut<tiny_glade::gui::wall_selection_tags::GiftTagContent>", "constructor": "toolbar::mei::core::impl$21::as_any_mut", "arguments": ["tiny_glade::gui::wall_selection_tags::GiftTagContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::impl$21::as_any<tiny_glade::gui::wall_selection_tags::GiftTagContent>", "constructor": "toolbar::mei::core::impl$21::as_any", "arguments": ["tiny_glade::gui::wall_selection_tags::GiftTagContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::impl$21::type_name<tiny_glade::gui::wall_selection_tags::GiftTagContent>", "constructor": "toolbar::mei::core::impl$21::type_name", "arguments": ["tiny_glade::gui::wall_selection_tags::GiftTagContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::NodeContent::pre_update<tiny_glade::gui::wall_selection_tags::GiftTagContent>", "constructor": "toolbar::mei::core::NodeContent::pre_update", "arguments": ["tiny_glade::gui::wall_selection_tags::GiftTagContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::NodeContent::interactive<tiny_glade::gui::wall_selection_tags::GiftTagContent>", "constructor": "toolbar::mei::core::NodeContent::interactive", "arguments": ["tiny_glade::gui::wall_selection_tags::GiftTagContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::NodeContent::process_input_action<tiny_glade::gui::wall_selection_tags::GiftTagContent>", "constructor": "toolbar::mei::core::NodeContent::process_input_action", "arguments": ["tiny_glade::gui::wall_selection_tags::GiftTagContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::NodeContent::on_input<tiny_glade::gui::wall_selection_tags::GiftTagContent>", "constructor": "toolbar::mei::core::NodeContent::on_input", "arguments": ["tiny_glade::gui::wall_selection_tags::GiftTagContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: wall_selection_tags.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: wall_selection_tags.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::mode_manager::WallSelectionGroups>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::PreviewWallSelectionGroup>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::ExpandWallSelectionGroup>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemGroupWasBookmarked>,tuple$<bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::onboarding::OnboardingDisabled> > >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>", "bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>", "bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>", "bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>", "bevy_ecs::change_detection::ResMut<country_core::systems::mode_manager::WallSelectionGroups>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::PreviewWallSelectionGroup>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::ExpandWallSelectionGroup>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemGroupWasBookmarked>", "tuple$<bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::onboarding::OnboardingDisabled> > >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::mode_manager::WallSelectionGroups>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::PreviewWallSelectionGroup>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::ExpandWallSelectionGroup>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemGroupWasBookmarked>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::onboarding::OnboardingDisabled>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::wall_selection_tags::gift_tag::closure$0
// 信息 ID: 164cc346025d05aa868bec8c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::gui::wall_selection_tags::gift_tag", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::gui::wall_selection_tags::gift_tag<u64>; module=196, inline_record=720; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::gui::wall_selection_tags::gift_tag<u64>; module=196, inline_record=748; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::gui::wall_selection_tags::gift_tag<u64>; module=196, inline_record=776; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::gui::wall_selection_tags::gift_tag<ref$<str$> >; module=196, inline_record=1416; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::gui::wall_selection_tags::gift_tag<ref$<str$> >; module=196, inline_record=1444; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::gui::wall_selection_tags::gift_tag<ref$<str$> >; module=196, inline_record=1472; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::wall_selection_tags::gift_tag::closure$1
// 信息 ID: 6feb627480fb72160ba8c0cc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::gui::wall_selection_tags::gift_tag", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::gui::wall_selection_tags::gift_tag<u64>; module=196, inline_record=1008; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::gui::wall_selection_tags::gift_tag<ref$<str$> >; module=196, inline_record=1704; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::wall_selection_tags::gift_tag<ref$<str$> >
// 信息 ID: 9bf5aedbb9382633e369a874
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] tiny_glade::gui::wall_selection_tags::gift_tag -> ["ref$<str$>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::wall_selection_tags::wall_selection_tags @ 0x6727c6 -> 0x557bc0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::wall_selection_tags::gift_tag<u64>
// 信息 ID: bec6afc1ebb4e94d4726063f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tiny_glade::gui::wall_selection_tags::gift_tag -> ["u64"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::wall_selection_tags::wall_selection_tags @ 0x672b35 -> 0x557760 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::wall_selection_tags::impl$0::create_node_content
// 信息 ID: 1531015833da730cbd286be7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::core::NodeContext::get_content_mut_impl<tiny_glade::gui::wall_selection_tags::GiftTagContent>; module=67, inline_record=11276; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::wall_selection_tags::impl$1::prepare_layout
// 信息 ID: a2421ce05cad1c75dde29a15
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::wall_selection_tags::impl$1::render
// 信息 ID: e2c0e270ff3b543dbd94d579
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::wall_selection_tags::impl$2::default
// 信息 ID: c65d5b03c0546b38f80b78ba
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::core::NodeContext::get_content_mut_impl<tiny_glade::gui::wall_selection_tags::GiftTagContent>; module=67, inline_record=11304; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::wall_selection_tags::wall_selection_tags
// 信息 ID: 88b79c2a0ebcacdfa3e79bec
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] e4db5074daee5e81a36ceceb: void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::mode_manager::WallSelectionGroups>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::PreviewWallSelectionGroup>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::ExpandWallSelectionGroup>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemGroupWasBookmarked>,tuple$<bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::onboarding::OnboardingDisabled> > >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::mode_manager::WallSelectionGroups>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::PreviewWallSelectionGroup>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::ExpandWallSelectionGroup>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemGroupWasBookmarked>,tuple$<bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::onboarding::OnboardingDisabled> > >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::mode_manager::WallSelectionGroups>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::PreviewWallSelectionGroup>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::ExpandWallSelectionGroup>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemGroupWasBookmarked>,tuple$<bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::onboarding::OnboardingDisabled> > >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >)> > @ 0x2b13d7 -> 0x671f70 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::mode_manager::WallSelectionGroups>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::PreviewWallSelectionGroup>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::ExpandWallSelectionGroup>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemGroupWasBookmarked>,tuple$<bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::onboarding::OnboardingDisabled> > >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::mode_manager::WallSelectionGroups>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::PreviewWallSelectionGroup>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::ExpandWallSelectionGroup>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemGroupWasBookmarked>,tuple$<bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::onboarding::OnboardingDisabled> > >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >)> @ 0x558f66 -> 0x671f70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::wall_selection_tags::wall_selection_tags::closure$0
// 信息 ID: 670dc5ec2fd9b5eb3559e679
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::gui::wall_selection_tags::wall_selection_tags", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::gui::wall_selection_tags::wall_selection_tags; module=246, inline_record=14180; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::wall_selection_tags::wall_selection_tags::closure$1
// 信息 ID: ff7a944ecd48161fd00eee15
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::gui::wall_selection_tags::wall_selection_tags", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::gui::wall_selection_tags::wall_selection_tags; module=246, inline_record=14592; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
