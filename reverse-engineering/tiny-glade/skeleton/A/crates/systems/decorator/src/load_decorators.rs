// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/decorator/src/load_decorators.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 6e0da583c820f77ac2a9694e61ff690f540ea287c79d4e4f25c68ebb2b7161f6
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: load_decorators.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 3849
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"PDB_registry_path_only","version":"1.0.86"}]
// use bevy_ecs; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use enum_iterator; // namespace candidate
//   package identities: [{"name":"enum-iterator","source":"PDB_registry_path_only","version":"1.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use std; // namespace candidate
//   package identities: []
// use system_decorator; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_decorator::load_decorators::load_decorators
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3849,"record_offset":988,"rva":29636960,"va":5398346080}
// 编译实例 RVAs: 0x1c43960
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$std..fs..ReadDir$u20$as$u20$core..iter..traits..iterator..Iterator$GT$::next
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20938208","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398346080,"raw_hex":"e82082c0fe","size":5,"soft_fail":false,"target_va":5377408080,"va":5398346283,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$std..sys..pal..windows..fs..FindNextFileHandle$u20$as$u20$core..ops..drop..Drop$GT$::drop
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20881478","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398346080,"raw_hex":"e8ba5fc1fe","size":5,"soft_fail":false,"target_va":5377465440,"va":5398346913,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003390
//     owner=unresolved_namespace; count=1
//     target_aliases=1000 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-29624632","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398346080,"raw_hex":"e8c8f63bfe","size":5,"soft_fail":false,"target_va":5368722320,"va":5398346947,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402cfa00
//     owner=unresolved_namespace; count=5
//     target_aliases=3 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26690169","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398346080,"raw_hex":"e887bd68fe","size":5,"soft_fail":false,"target_va":5371656704,"va":5398346868,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14036ef60
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26054141","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398346080,"raw_hex":"e8037272fe","size":5,"soft_fail":false,"target_va":5372309344,"va":5398363480,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140370260
//     owner=unresolved_namespace; count=2
//     target_aliases=89 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26043772","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398346080,"raw_hex":"e8849a72fe","size":5,"soft_fail":false,"target_va":5372314208,"va":5398357975,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=13
//     target_aliases=1 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25598573","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398346080,"raw_hex":"e8936579fe","size":5,"soft_fail":false,"target_va":5372749696,"va":5398348264,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=9
//     target_aliases=1 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25597132","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398346080,"raw_hex":"e8346b79fe","size":5,"soft_fail":false,"target_va":5372749712,"va":5398346839,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140593250
//     owner=unresolved_namespace; count=18
//     target_aliases=15 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23792377","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398346080,"raw_hex":"e807f594fe","size":5,"soft_fail":false,"target_va":5374554704,"va":5398347076,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b3820
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23659767","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398346080,"raw_hex":"e809fb96fe","size":5,"soft_fail":false,"target_va":5374687264,"va":5398347026,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406299f0
//     owner=unresolved_namespace; count=1
//     target_aliases=28 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23175314","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398346080,"raw_hex":"e86e5f9efe","size":5,"soft_fail":false,"target_va":5375171056,"va":5398346365,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140633130
//     owner=unresolved_namespace; count=18
//     target_aliases=2 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23137315","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398346080,"raw_hex":"e8ddf39efe","size":5,"soft_fail":false,"target_va":5375209776,"va":5398347086,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ef6c0
//     owner=unresolved_namespace; count=33
//     target_aliases=2 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22377363","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398346080,"raw_hex":"e86d8caafe","size":5,"soft_fail":false,"target_va":5375981248,"va":5398358606,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14084b050
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20941380","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398346080,"raw_hex":"e8bc75c0fe","size":5,"soft_fail":false,"target_va":5377405008,"va":5398346383,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cf9840
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16033218","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398346080,"raw_hex":"e83e5a0bff","size":5,"soft_fail":false,"target_va":5382314048,"va":5398347261,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d02460
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16008349","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398346080,"raw_hex":"e863bb0bff","size":5,"soft_fail":false,"target_va":5382349920,"va":5398358264,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f1f1f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13781891","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398346080,"raw_hex":"e87db42dff","size":5,"soft_fail":false,"target_va":5384565232,"va":5398347118,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141e63de0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2228915","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398346080,"raw_hex":"e8b3022200","size":5,"soft_fail":false,"target_va":5400575456,"va":5398346536,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5247156","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398346080,"raw_hex":"e8b4105000","size":5,"soft_fail":false,"target_va":5403593360,"va":5398346199,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=13
//     evidence={"assembly":"\tcallq\t4621222","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398346080,"raw_hex":"e8a6834600","size":5,"soft_fail":false,"target_va":5402978144,"va":5398356917,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-20816864","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398346080,"raw_hex":"e8205cc2fe","size":5,"soft_fail":false,"target_va":5377529888,"va":5398346747,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::construct::hc831513920bfd8ad (.llvm.9425496465068723702)
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5230219","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398346080,"raw_hex":"e88bce4f00","size":5,"soft_fail":false,"target_va":5403588064,"va":5398357840,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-20993690","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398346080,"raw_hex":"e866a9bffe","size":5,"soft_fail":false,"target_va":5377364336,"va":5398358021,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t4621980","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398346080,"raw_hex":"e89c864600","size":5,"soft_fail":false,"target_va":5402978880,"va":5398356895,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=9
//     evidence={"assembly":"\tcallq\t4622129","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398346080,"raw_hex":"e831874600","size":5,"soft_fail":false,"target_va":5402979152,"va":5398357018,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<country_core::resources::walls::decorator_outline::DecoratorOutline> >
//     owner=rust_standard_library; count=19
//     evidence={"assembly":"\tcallq\t-23149101","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398346080,"raw_hex":"e8d3c59efe","size":5,"soft_fail":false,"target_va":5375209392,"va":5398358488,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t4623134","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398346080,"raw_hex":"e81e8b4600","size":5,"soft_fail":false,"target_va":5402980128,"va":5398356989,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::mesh_library::AssetMeshLibrary::get_by_name
//     owner=unresolved_namespace; count=10
//     evidence={"assembly":"\tcallq\t-23781643","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398346080,"raw_hex":"e8f51e95fe","size":5,"soft_fail":false,"target_va":5374567936,"va":5398349574,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::geometry::sphere::Sphere::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-24155573","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398346080,"raw_hex":"e84b6a8ffe","size":5,"soft_fail":false,"target_va":5374191536,"va":5398347104,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorType::max_rank
//     owner=unresolved_namespace; count=10
//     evidence={"assembly":"\tcallq\t-23776642","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398346080,"raw_hex":"e87e3295fe","size":5,"soft_fail":false,"target_va":5374570784,"va":5398347421,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::insert
//     owner=unresolved_namespace; count=14
//     evidence={"assembly":"\tcallq\t-23137404","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398346080,"raw_hex":"e884f39efe","size":5,"soft_fail":false,"target_va":5375210704,"va":5398348103,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::insert_collision_and_interaction_as_same
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t-23138680","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398346080,"raw_hex":"e888ee9efe","size":5,"soft_fail":false,"target_va":5375209792,"va":5398348467,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::startup::load_mesh_into_library
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-23219582","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398346080,"raw_hex":"e882b29dfe","size":5,"soft_fail":false,"target_va":5375127232,"va":5398346809,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::backtrace::Backtrace::capture
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-20962214","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398346080,"raw_hex":"e85a24c0fe","size":5,"soft_fail":false,"target_va":5377395552,"va":5398357761,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirEntry::file_name
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-20937642","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398346080,"raw_hex":"e85684c0fe","size":5,"soft_fail":false,"target_va":5377408720,"va":5398346357,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirEntry::path
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-20938298","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398346080,"raw_hex":"e8c681c0fe","size":5,"soft_fail":false,"target_va":5377408224,"va":5398346517,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::pal::windows::fs::readdir
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-20877936","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398346080,"raw_hex":"e8906dc1fe","size":5,"soft_fail":false,"target_va":5377468224,"va":5398346155,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::load_decorators::load_collision_as_bbx
//     owner=unresolved_namespace; count=36
//     evidence={"assembly":"\tcallq\t17833","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398346080,"raw_hex":"e8a9450000","size":5,"soft_fail":false,"target_va":5398366080,"va":5398348242,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::load_decorators::load_decorators::closure$0
//     owner=unresolved_namespace; count=11
//     evidence={"assembly":"\tcallq\t16365","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398346080,"raw_hex":"e8ed3f0000","size":5,"soft_fail":false,"target_va":5398363600,"va":5398347230,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::load_decorators::load_decorators::closure$1
//     owner=unresolved_namespace; count=36
//     evidence={"assembly":"\tcallq\t17344","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398346080,"raw_hex":"e8c0430000","size":5,"soft_fail":false,"target_va":5398364736,"va":5398347387,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::load_decorators::load_decorators::closure$3
//     owner=unresolved_namespace; count=20
//     evidence={"assembly":"\tcallq\t17752","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398346080,"raw_hex":"e858450000","size":5,"soft_fail":false,"target_va":5398365328,"va":5398347571,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":3849,"record_offset":1812,"parent_record_offset":988,"ancestor_record_offset":988,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":3849,"record_offset":5092,"parent_record_offset":988,"ancestor_record_offset":988,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/context.rs
//     evidence={"module":3849,"record_offset":14976,"parent_record_offset":988,"ancestor_record_offset":988,"inlinee":10988,"source_line":46,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=19
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3849,"record_offset":3172,"parent_record_offset":988,"ancestor_record_offset":988,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":3849,"record_offset":6904,"parent_record_offset":988,"ancestor_record_offset":988,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$15::next
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":3849,"record_offset":4704,"parent_record_offset":988,"ancestor_record_offset":988,"inlinee":7622,"source_line":1248,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3849,"record_offset":1492,"parent_record_offset":988,"ancestor_record_offset":988,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3849,"record_offset":1896,"parent_record_offset":988,"ancestor_record_offset":988,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3849,"record_offset":1256,"parent_record_offset":988,"ancestor_record_offset":988,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::strip_suffix
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":3849,"record_offset":1528,"parent_record_offset":988,"ancestor_record_offset":988,"inlinee":22131,"source_line":2244,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_direct] enum_iterator::all
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"enum-iterator","version":"1.2.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/enum-iterator-1.2.0/src/lib.rs
//     evidence={"module":3849,"record_offset":3080,"parent_record_offset":988,"ancestor_record_offset":988,"inlinee":26607,"source_line":100,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_direct] enum_iterator::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"enum-iterator","version":"1.2.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/enum-iterator-1.2.0/src/lib.rs
//     evidence={"module":3849,"record_offset":3108,"parent_record_offset":988,"ancestor_record_offset":988,"inlinee":23004,"source_line":221,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_direct] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":3849,"record_offset":1432,"parent_record_offset":988,"ancestor_record_offset":988,"inlinee":6730,"source_line":812,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_direct] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":3849,"record_offset":1376,"parent_record_offset":988,"ancestor_record_offset":988,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_dir
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":3849,"record_offset":1188,"parent_record_offset":988,"ancestor_record_offset":988,"inlinee":8478,"source_line":2465,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::load_decorators::load_decorators::closure$2
//     owner=private_source_ownership_candidate; count=11
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/load_decorators.rs
//     evidence={"module":3849,"record_offset":5888,"parent_record_offset":988,"ancestor_record_offset":988,"inlinee":111798,"source_line":74,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::load_decorators::load_decorators::closure$4
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/load_decorators.rs
//     evidence={"module":3849,"record_offset":3228,"parent_record_offset":988,"ancestor_record_offset":988,"inlinee":111797,"source_line":92,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3849,"record_offset":5376,"parent_record_offset":5352,"ancestor_record_offset":988,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3849,"record_offset":5400,"parent_record_offset":5376,"ancestor_record_offset":988,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3849,"record_offset":2048,"parent_record_offset":2024,"ancestor_record_offset":988,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3849,"record_offset":5352,"parent_record_offset":5324,"ancestor_record_offset":988,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3849,"record_offset":2024,"parent_record_offset":1968,"ancestor_record_offset":988,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":3849,"record_offset":1860,"parent_record_offset":1836,"ancestor_record_offset":988,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3849,"record_offset":1996,"parent_record_offset":1968,"ancestor_record_offset":988,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3849,"record_offset":5324,"parent_record_offset":5296,"ancestor_record_offset":988,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3849,"record_offset":5296,"parent_record_offset":5268,"ancestor_record_offset":988,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3849,"record_offset":5472,"parent_record_offset":5296,"ancestor_record_offset":988,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3849,"record_offset":1968,"parent_record_offset":1944,"ancestor_record_offset":988,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3849,"record_offset":5232,"parent_record_offset":5204,"ancestor_record_offset":988,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3849,"record_offset":5204,"parent_record_offset":5176,"ancestor_record_offset":988,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3849,"record_offset":5148,"parent_record_offset":5120,"ancestor_record_offset":988,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3849,"record_offset":5176,"parent_record_offset":5148,"ancestor_record_offset":988,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3849,"record_offset":5120,"parent_record_offset":5092,"ancestor_record_offset":988,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":3849,"record_offset":6960,"parent_record_offset":6932,"ancestor_record_offset":988,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":3849,"record_offset":6932,"parent_record_offset":6904,"ancestor_record_offset":988,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":3849,"record_offset":2204,"parent_record_offset":2168,"ancestor_record_offset":988,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":3849,"record_offset":2168,"parent_record_offset":2144,"ancestor_record_offset":988,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3849,"record_offset":5268,"parent_record_offset":5232,"ancestor_record_offset":988,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/error.rs
//     evidence={"module":3849,"record_offset":15048,"parent_record_offset":15004,"ancestor_record_offset":988,"inlinee":5381,"source_line":173,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/context.rs
//     evidence={"module":3849,"record_offset":15004,"parent_record_offset":14976,"ancestor_record_offset":988,"inlinee":5405,"source_line":23,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=19
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3849,"record_offset":3196,"parent_record_offset":3172,"ancestor_record_offset":988,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3849,"record_offset":4800,"parent_record_offset":4732,"ancestor_record_offset":988,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3849,"record_offset":1640,"parent_record_offset":1612,"ancestor_record_offset":988,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":3849,"record_offset":5532,"parent_record_offset":5508,"ancestor_record_offset":988,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$14::spec_next
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":3849,"record_offset":4732,"parent_record_offset":4704,"ancestor_record_offset":988,"inlinee":7624,"source_line":1153,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3849,"record_offset":3804,"parent_record_offset":3772,"ancestor_record_offset":988,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::is_empty
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":3849,"record_offset":4768,"parent_record_offset":4732,"ancestor_record_offset":988,"inlinee":7625,"source_line":545,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3849,"record_offset":1836,"parent_record_offset":1812,"ancestor_record_offset":988,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3849,"record_offset":5916,"parent_record_offset":5888,"ancestor_record_offset":988,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$40::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3849,"record_offset":3140,"parent_record_offset":3108,"ancestor_record_offset":988,"inlinee":5366,"source_line":2485,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3849,"record_offset":5508,"parent_record_offset":5232,"ancestor_record_offset":988,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=49
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3849,"record_offset":1300,"parent_record_offset":1256,"ancestor_record_offset":988,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3849,"record_offset":2236,"parent_record_offset":2204,"ancestor_record_offset":988,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3849,"record_offset":5428,"parent_record_offset":5400,"ancestor_record_offset":988,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3849,"record_offset":1216,"parent_record_offset":1188,"ancestor_record_offset":988,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3849,"record_offset":1460,"parent_record_offset":1432,"ancestor_record_offset":988,"inlinee":4772,"source_line":631,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3849,"record_offset":3288,"parent_record_offset":3228,"ancestor_record_offset":988,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":3849,"record_offset":1664,"parent_record_offset":1640,"ancestor_record_offset":988,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":3849,"record_offset":1688,"parent_record_offset":1664,"ancestor_record_offset":988,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::ends_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3849,"record_offset":1612,"parent_record_offset":1584,"ancestor_record_offset":988,"inlinee":5504,"source_line":2585,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::is_suffix_of
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3849,"record_offset":1584,"parent_record_offset":1556,"ancestor_record_offset":988,"inlinee":12627,"source_line":1005,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::strip_suffix_of
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3849,"record_offset":1556,"parent_record_offset":1528,"ancestor_record_offset":988,"inlinee":22133,"source_line":1011,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3849,"record_offset":2264,"parent_record_offset":2204,"ancestor_record_offset":988,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3849,"record_offset":2296,"parent_record_offset":2168,"ancestor_record_offset":988,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3849,"record_offset":3648,"parent_record_offset":3624,"ancestor_record_offset":988,"inlinee":5572,"source_line":3669,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":3849,"record_offset":1400,"parent_record_offset":1376,"ancestor_record_offset":988,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::load_decorators::load_decorators::closure$0
// 记录形式: compiled_procedure; instances=1
// 源码行: 49
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3849,"record_offset":16140,"rva":29654480,"va":5398363600}
// 编译实例 RVAs: 0x1c47dd0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14038b460
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25939044","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398363600,"raw_hex":"e89c3374fe","size":5,"soft_fail":false,"target_va":5372425312,"va":5398364351,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25614366","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398363600,"raw_hex":"e8e22779fe","size":5,"soft_fail":false,"target_va":5372749696,"va":5398364057,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25614577","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398363600,"raw_hex":"e80f2779fe","size":5,"soft_fail":false,"target_va":5372749712,"va":5398364284,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76e80
//     owner=unresolved_namespace; count=2
//     target_aliases=5 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15536034","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398363600,"raw_hex":"e85ef012ff","size":5,"soft_fail":false,"target_va":5382827648,"va":5398363677,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417afe30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4817021","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398363600,"raw_hex":"e8837fb6ff","size":5,"soft_fail":false,"target_va":5393546800,"va":5398363816,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4613924","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398363600,"raw_hex":"e824674600","size":5,"soft_fail":false,"target_va":5402978144,"va":5398364215,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::arrayed_indices
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-13795997","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398363600,"raw_hex":"e8637d2dff","size":5,"soft_fail":false,"target_va":5384567648,"va":5398363640,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::as_triangles
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-13793086","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398363600,"raw_hex":"e8c2882dff","size":5,"soft_fail":false,"target_va":5384570848,"va":5398363929,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::collision_bbx
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-13795204","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398363600,"raw_hex":"e87c802dff","size":5,"soft_fail":false,"target_va":5384568752,"va":5398363951,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::normals_without_indices
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-13794033","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398363600,"raw_hex":"e80f852dff","size":5,"soft_fail":false,"target_va":5384569664,"va":5398363692,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::positions_without_indices
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-13794332","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398363600,"raw_hex":"e8e4832dff","size":5,"soft_fail":false,"target_va":5384569328,"va":5398363655,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::uvs_without_indices
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-13793771","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398363600,"raw_hex":"e815862dff","size":5,"soft_fail":false,"target_va":5384570016,"va":5398363782,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::collision::world::RaycastMeshData::new_indexed
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-23847224","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398363600,"raw_hex":"e8c81e94fe","size":5,"soft_fail":false,"target_va":5374516688,"va":5398363907,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":3849,"record_offset":16588,"parent_record_offset":16140,"ancestor_record_offset":16140,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3849,"record_offset":16300,"parent_record_offset":16140,"ancestor_record_offset":16140,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3849,"record_offset":16356,"parent_record_offset":16140,"ancestor_record_offset":16140,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3849,"record_offset":16844,"parent_record_offset":16140,"ancestor_record_offset":16140,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3849,"record_offset":16712,"parent_record_offset":16688,"ancestor_record_offset":16140,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3849,"record_offset":16736,"parent_record_offset":16712,"ancestor_record_offset":16140,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3849,"record_offset":17020,"parent_record_offset":16996,"ancestor_record_offset":16140,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3849,"record_offset":16656,"parent_record_offset":16620,"ancestor_record_offset":16140,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3849,"record_offset":16688,"parent_record_offset":16656,"ancestor_record_offset":16140,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3849,"record_offset":16996,"parent_record_offset":16896,"ancestor_record_offset":16140,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":3849,"record_offset":16620,"parent_record_offset":16588,"ancestor_record_offset":16140,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3849,"record_offset":16928,"parent_record_offset":16896,"ancestor_record_offset":16140,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3849,"record_offset":16896,"parent_record_offset":16868,"ancestor_record_offset":16140,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3849,"record_offset":16324,"parent_record_offset":16300,"ancestor_record_offset":16140,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3849,"record_offset":16404,"parent_record_offset":16384,"ancestor_record_offset":16140,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3849,"record_offset":16960,"parent_record_offset":16928,"ancestor_record_offset":16140,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3849,"record_offset":16808,"parent_record_offset":16620,"ancestor_record_offset":16140,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3849,"record_offset":16764,"parent_record_offset":16736,"ancestor_record_offset":16140,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::load_decorators::load_decorators::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/load_decorators.rs
//     evidence={"module":3849,"record_offset":16384,"parent_record_offset":16356,"ancestor_record_offset":16140,"inlinee":111800,"source_line":54,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::load_decorators::load_decorators::closure$0::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/load_decorators.rs
//     evidence={"module":3849,"record_offset":16500,"parent_record_offset":16468,"ancestor_record_offset":16140,"inlinee":111801,"source_line":55,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::load_decorators::load_decorators::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 54
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B4B8
// 首条位置: {"module":3849,"inlinee":111800}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3849,"record_offset":16404,"parent_record_offset":16384,"ancestor_record_offset":16384,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3849,"record_offset":16428,"parent_record_offset":16404,"ancestor_record_offset":16384,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::load_decorators::load_decorators::closure$0::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 55
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B4B9
// 首条位置: {"module":3849,"inlinee":111801}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3849,"record_offset":16524,"parent_record_offset":16500,"ancestor_record_offset":16500,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3849,"record_offset":16548,"parent_record_offset":16524,"ancestor_record_offset":16500,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::load_decorators::load_decorators::closure$1
// 记录形式: compiled_procedure; instances=1
// 源码行: 65
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3849,"record_offset":17740,"rva":29655616,"va":5398364736}
// 编译实例 RVAs: 0x1c48240
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25615555","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398364736,"raw_hex":"e83d2379fe","size":5,"soft_fail":false,"target_va":5372749712,"va":5398365262,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d02460
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16015064","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398364736,"raw_hex":"e828a10bff","size":5,"soft_fail":false,"target_va":5382349920,"va":5398364979,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76e80
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15537327","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398364736,"raw_hex":"e851eb12ff","size":5,"soft_fail":false,"target_va":5382827648,"va":5398364970,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141e64a60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2213790","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398364736,"raw_hex":"e89ec72100","size":5,"soft_fail":false,"target_va":5400578656,"va":5398364861,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-20834966","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398364736,"raw_hex":"e86a15c2fe","size":5,"soft_fail":false,"target_va":5377529888,"va":5398364849,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-27058865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398364736,"raw_hex":"e84f1d63fe","size":5,"soft_fail":false,"target_va":5371306160,"va":5398365020,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21000972","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398364736,"raw_hex":"e8f48cbffe","size":5,"soft_fail":false,"target_va":5377364336,"va":5398365303,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4614243","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398364736,"raw_hex":"e863684600","size":5,"soft_fail":false,"target_va":5402979344,"va":5398365096,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4614977","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398364736,"raw_hex":"e8416b4600","size":5,"soft_fail":false,"target_va":5402980128,"va":5398365146,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::positions_without_indices
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-13795632","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398364736,"raw_hex":"e8d07e2dff","size":5,"soft_fail":false,"target_va":5384569328,"va":5398364955,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":3849,"record_offset":17908,"parent_record_offset":17740,"ancestor_record_offset":17740,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3849,"record_offset":18220,"parent_record_offset":17740,"ancestor_record_offset":17740,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3849,"record_offset":18124,"parent_record_offset":17740,"ancestor_record_offset":17740,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3849,"record_offset":18060,"parent_record_offset":17740,"ancestor_record_offset":17740,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3849,"record_offset":18352,"parent_record_offset":17740,"ancestor_record_offset":17740,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3849,"record_offset":17988,"parent_record_offset":17740,"ancestor_record_offset":17740,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3849,"record_offset":18528,"parent_record_offset":18504,"ancestor_record_offset":17740,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3849,"record_offset":18504,"parent_record_offset":18404,"ancestor_record_offset":17740,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":3849,"record_offset":17956,"parent_record_offset":17932,"ancestor_record_offset":17740,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3849,"record_offset":18436,"parent_record_offset":18404,"ancestor_record_offset":17740,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3849,"record_offset":18288,"parent_record_offset":18252,"ancestor_record_offset":17740,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3849,"record_offset":18404,"parent_record_offset":18376,"ancestor_record_offset":17740,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3849,"record_offset":18252,"parent_record_offset":18220,"ancestor_record_offset":17740,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3849,"record_offset":18088,"parent_record_offset":18060,"ancestor_record_offset":17740,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3849,"record_offset":18468,"parent_record_offset":18436,"ancestor_record_offset":17740,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3849,"record_offset":17932,"parent_record_offset":17908,"ancestor_record_offset":17740,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3849,"record_offset":18028,"parent_record_offset":17988,"ancestor_record_offset":17740,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3849,"record_offset":18320,"parent_record_offset":18220,"ancestor_record_offset":17740,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3849,"record_offset":18152,"parent_record_offset":18124,"ancestor_record_offset":17740,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3849,"record_offset":18180,"parent_record_offset":18152,"ancestor_record_offset":17740,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::load_decorators::load_decorators::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=12
// 源码行: 74
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B4B6
// 首条位置: {"module":3849,"inlinee":111798}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3849,"record_offset":5916,"parent_record_offset":5888,"ancestor_record_offset":5888,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::load_decorators::load_decorators::closure$3
// 记录形式: compiled_procedure; instances=1
// 源码行: 79
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3849,"record_offset":18576,"rva":29656208,"va":5398365328}
// 编译实例 RVAs: 0x1c48490
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14038b460
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25940676","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398365328,"raw_hex":"e83c2d74fe","size":5,"soft_fail":false,"target_va":5372425312,"va":5398365983,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25615917","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398365328,"raw_hex":"e8d32179fe","size":5,"soft_fail":false,"target_va":5372749696,"va":5398365608,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25615757","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398365328,"raw_hex":"e8732279fe","size":5,"soft_fail":false,"target_va":5372749712,"va":5398365464,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76e80
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15537762","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398365328,"raw_hex":"e89ee912ff","size":5,"soft_fail":false,"target_va":5382827648,"va":5398365405,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4612370","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398365328,"raw_hex":"e812614600","size":5,"soft_fail":false,"target_va":5402978144,"va":5398365769,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4613121","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398365328,"raw_hex":"e801644600","size":5,"soft_fail":false,"target_va":5402978880,"va":5398365754,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::mesh_library::AssetMeshLibrary::get_by_name
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-23797439","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398365328,"raw_hex":"e841e194fe","size":5,"soft_fail":false,"target_va":5374567936,"va":5398365370,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::as_triangles
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-13794643","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398365328,"raw_hex":"e8ad822dff","size":5,"soft_fail":false,"target_va":5384570848,"va":5398365486,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::collision_bbx
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-13796758","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398365328,"raw_hex":"e86a7a2dff","size":5,"soft_fail":false,"target_va":5384568752,"va":5398365505,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::positions_without_indices
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-13796067","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398365328,"raw_hex":"e81d7d2dff","size":5,"soft_fail":false,"target_va":5384569328,"va":5398365390,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::collision::world::RaycastMeshData::new_convex
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-23848922","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398365328,"raw_hex":"e8261894fe","size":5,"soft_fail":false,"target_va":5374516512,"va":5398365429,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":3849,"record_offset":19024,"parent_record_offset":18576,"ancestor_record_offset":18576,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3849,"record_offset":18772,"parent_record_offset":18576,"ancestor_record_offset":18576,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3849,"record_offset":18736,"parent_record_offset":18576,"ancestor_record_offset":18576,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3849,"record_offset":18828,"parent_record_offset":18576,"ancestor_record_offset":18576,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3849,"record_offset":19148,"parent_record_offset":19124,"ancestor_record_offset":18576,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3849,"record_offset":19172,"parent_record_offset":19148,"ancestor_record_offset":18576,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3849,"record_offset":18980,"parent_record_offset":18956,"ancestor_record_offset":18576,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3849,"record_offset":19092,"parent_record_offset":19056,"ancestor_record_offset":18576,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3849,"record_offset":19124,"parent_record_offset":19092,"ancestor_record_offset":18576,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3849,"record_offset":18956,"parent_record_offset":18868,"ancestor_record_offset":18576,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":3849,"record_offset":19056,"parent_record_offset":19024,"ancestor_record_offset":18576,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3849,"record_offset":18892,"parent_record_offset":18868,"ancestor_record_offset":18576,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3849,"record_offset":18868,"parent_record_offset":18848,"ancestor_record_offset":18576,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3849,"record_offset":18796,"parent_record_offset":18772,"ancestor_record_offset":18576,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3849,"record_offset":18920,"parent_record_offset":18892,"ancestor_record_offset":18576,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3849,"record_offset":18848,"parent_record_offset":18828,"ancestor_record_offset":18576,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3849,"record_offset":19200,"parent_record_offset":19172,"ancestor_record_offset":18576,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::load_decorators::load_decorators::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 92
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B4B5
// 首条位置: {"module":3849,"inlinee":111797}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3849,"record_offset":3544,"parent_record_offset":3228,"ancestor_record_offset":3228,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3849,"record_offset":3288,"parent_record_offset":3228,"ancestor_record_offset":3228,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3849,"record_offset":3492,"parent_record_offset":3468,"ancestor_record_offset":3228,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3849,"record_offset":3468,"parent_record_offset":3412,"ancestor_record_offset":3228,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3849,"record_offset":3440,"parent_record_offset":3412,"ancestor_record_offset":3228,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3849,"record_offset":3412,"parent_record_offset":3384,"ancestor_record_offset":3228,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3849,"record_offset":3804,"parent_record_offset":3772,"ancestor_record_offset":3228,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3849,"record_offset":3332,"parent_record_offset":3288,"ancestor_record_offset":3228,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3849,"record_offset":3648,"parent_record_offset":3624,"ancestor_record_offset":3228,"inlinee":5572,"source_line":3669,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::load_decorators::load_collision_as_bbx
// 记录形式: compiled_procedure; instances=1
// 源码行: 614
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3849,"record_offset":19712,"rva":29656960,"va":5398366080}
// 编译实例 RVAs: 0x1c48780
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402ce6a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26715476","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398366080,"raw_hex":"e8ac5a68fe","size":5,"soft_fail":false,"target_va":5371651744,"va":5398367215,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14038b460
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25941744","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398366080,"raw_hex":"e8102974fe","size":5,"soft_fail":false,"target_va":5372425312,"va":5398367051,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25616945","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398366080,"raw_hex":"e8cf1d79fe","size":5,"soft_fail":false,"target_va":5372749696,"va":5398366636,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25617483","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398366080,"raw_hex":"e8b51b79fe","size":5,"soft_fail":false,"target_va":5372749712,"va":5398367190,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da780
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21422326","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398366080,"raw_hex":"e80a1fb9fe","size":5,"soft_fail":false,"target_va":5376944000,"va":5398366321,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4611175","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398366080,"raw_hex":"e8675c4600","size":5,"soft_fail":false,"target_va":5402978144,"va":5398366964,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t4611971","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398366080,"raw_hex":"e8835f4600","size":5,"soft_fail":false,"target_va":5402978880,"va":5398366904,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5151590","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398366080,"raw_hex":"e8669b4e00","size":5,"soft_fail":false,"target_va":5403518544,"va":5398366949,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::mesh_library::AssetMeshLibrary::get_by_name
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-23798236","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398366080,"raw_hex":"e824de94fe","size":5,"soft_fail":false,"target_va":5374567936,"va":5398366167,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::collision_bbx
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-13797444","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398366080,"raw_hex":"e8bc772dff","size":5,"soft_fail":false,"target_va":5384568752,"va":5398366191,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::collision::world::RaycastMeshData::new_cuboid
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-23845322","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398366080,"raw_hex":"e8362694fe","size":5,"soft_fail":false,"target_va":5374521216,"va":5398366533,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::box_trigger_volume
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-24080340","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398366080,"raw_hex":"e82c9090fe","size":5,"soft_fail":false,"target_va":5374286080,"va":5398366415,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::xz_rotation
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21423417","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398366080,"raw_hex":"e8c71ab9fe","size":5,"soft_fail":false,"target_va":5376942944,"va":5398366356,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":3849,"record_offset":20200,"parent_record_offset":19712,"ancestor_record_offset":19712,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3849,"record_offset":19884,"parent_record_offset":19712,"ancestor_record_offset":19712,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3849,"record_offset":20496,"parent_record_offset":19712,"ancestor_record_offset":19712,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3849,"record_offset":20060,"parent_record_offset":19712,"ancestor_record_offset":19712,"inlinee":14690,"source_line":814,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$46::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3849,"record_offset":19920,"parent_record_offset":19712,"ancestor_record_offset":19712,"inlinee":5676,"source_line":18,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::center
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":3849,"record_offset":20032,"parent_record_offset":19712,"ancestor_record_offset":19712,"inlinee":22795,"source_line":60,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::height
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":3849,"record_offset":19948,"parent_record_offset":19712,"ancestor_record_offset":19712,"inlinee":21736,"source_line":65,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::length
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":3849,"record_offset":20004,"parent_record_offset":19712,"ancestor_record_offset":19712,"inlinee":45783,"source_line":75,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::width
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":3849,"record_offset":19976,"parent_record_offset":19712,"ancestor_record_offset":19712,"inlinee":21735,"source_line":70,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3849,"record_offset":20328,"parent_record_offset":20304,"ancestor_record_offset":19712,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3849,"record_offset":20352,"parent_record_offset":20328,"ancestor_record_offset":19712,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3849,"record_offset":20660,"parent_record_offset":20636,"ancestor_record_offset":19712,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3849,"record_offset":20272,"parent_record_offset":20236,"ancestor_record_offset":19712,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3849,"record_offset":20304,"parent_record_offset":20272,"ancestor_record_offset":19712,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3849,"record_offset":20636,"parent_record_offset":20544,"ancestor_record_offset":19712,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":3849,"record_offset":20236,"parent_record_offset":20200,"ancestor_record_offset":19712,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3849,"record_offset":20572,"parent_record_offset":20544,"ancestor_record_offset":19712,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3849,"record_offset":20544,"parent_record_offset":20520,"ancestor_record_offset":19712,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":3849,"record_offset":20728,"parent_record_offset":20704,"ancestor_record_offset":19712,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3849,"record_offset":20600,"parent_record_offset":20572,"ancestor_record_offset":19712,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3849,"record_offset":20424,"parent_record_offset":20236,"ancestor_record_offset":19712,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3849,"record_offset":20380,"parent_record_offset":20352,"ancestor_record_offset":19712,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3849,"record_offset":20764,"parent_record_offset":20728,"ancestor_record_offset":19712,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3849,"record_offset":20788,"parent_record_offset":20764,"ancestor_record_offset":19712,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/03849.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3849,"record_offset":20820,"parent_record_offset":20728,"ancestor_record_offset":19712,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/03849.jsonl.gz"}
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
// 完整结构及原始类型记录: load_decorators.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: load_decorators.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: load_decorators.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>

// ------------------------------------------------------------------
// 函数: system_decorator::load_decorators::load_collision_as_bbx
// 信息 ID: e4e72f699711e4da68e96637
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::load_decorators::load_decorators
// 信息 ID: f29fccdb91be2eea3c89fee0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] b852f1cfe42c3f93ad8d035a: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>)> > @ 0x17cccaa -> 0x1c43960 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>)> @ 0x17dde8f -> 0x1c43960 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::load_decorators::load_decorators::closure$0
// 信息 ID: be02d459966a9da033c7b57d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::load_decorators::load_decorators", "captures": "unknown"}
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::load_decorators::load_decorators @ 0x1c43dde -> 0x1c47dd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::load_decorators::load_decorators @ 0x1c462fa -> 0x1c47dd0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::load_decorators::load_decorators::closure$0::closure$0
// 信息 ID: 91ff84459fc01bf6c7a1d583
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::load_decorators::load_decorators", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::load_decorators::load_decorators::closure$0; module=3849, inline_record=16384; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::load_decorators::load_decorators::closure$0::closure$1
// 信息 ID: 96beb0be5547c0353d2ba569
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::load_decorators::load_decorators", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::load_decorators::load_decorators::closure$0; module=3849, inline_record=16500; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::load_decorators::load_decorators::closure$1
// 信息 ID: e31c4564de6ac483060e810d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_decorator::load_decorators::load_decorators", "captures": "unknown"}
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::load_decorators::load_decorators @ 0x1c43e7b -> 0x1c48240 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::load_decorators::load_decorators @ 0x1c46320 -> 0x1c48240 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::load_decorators::load_decorators @ 0x1c4625f -> 0x1c48240 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::load_decorators::load_decorators @ 0x1c4623d -> 0x1c48240 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::load_decorators::load_decorators::closure$2
// 信息 ID: 3f4b8ea728467e8e7b9434ea
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_decorator::load_decorators::load_decorators", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 11 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::load_decorators::load_decorators; module=3849, inline_record=5888; 不是独立调用边。
// [PDB 内联宿主] system_decorator::load_decorators::load_decorators; module=3849, inline_record=6004; 不是独立调用边。
// [PDB 内联宿主] system_decorator::load_decorators::load_decorators; module=3849, inline_record=6324; 不是独立调用边。
// [PDB 内联宿主] system_decorator::load_decorators::load_decorators; module=3849, inline_record=6384; 不是独立调用边。
// [PDB 内联宿主] system_decorator::load_decorators::load_decorators; module=3849, inline_record=8772; 不是独立调用边。
// [PDB 内联宿主] system_decorator::load_decorators::load_decorators; module=3849, inline_record=9392; 不是独立调用边。
// [PDB 内联宿主] system_decorator::load_decorators::load_decorators; module=3849, inline_record=10012; 不是独立调用边。
// [PDB 内联宿主] system_decorator::load_decorators::load_decorators; module=3849, inline_record=14668; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::load_decorators::load_decorators::closure$3
// 信息 ID: 4ab2805e1c4f04b0598f762c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "system_decorator::load_decorators::load_decorators", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::load_decorators::load_decorators::closure$4
// 信息 ID: 09d9a3e8735151679227a86b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "system_decorator::load_decorators::load_decorators", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::load_decorators::load_decorators; module=3849, inline_record=3228; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
