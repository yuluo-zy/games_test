// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/ui/ui_update.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 5435f31d8d7621b20a7c790723ca760e6b8a39be42e86091a8b544f13a2d1c59
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: ui_update.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 410
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
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"PDB_registry_path_only","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui::ui_update::ui_update
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":410,"record_offset":21876,"rva":5692784,"va":5374401904}
// 编译实例 RVAs: 0x56dd70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=5
//     target_aliases=9 candidates; complete list: ui_update.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4789722","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374401904,"raw_hex":"e826eab6ff","size":5,"soft_fail":false,"target_va":5369613520,"va":5374403237,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402cf080
//     owner=unresolved_namespace; count=13
//     target_aliases=2 candidates; complete list: ui_update.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2748709","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374401904,"raw_hex":"e8db0ed6ff","size":5,"soft_fail":false,"target_va":5371654272,"va":5374402976,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dec370
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_update.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8905751","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374401904,"raw_hex":"e817e48700","size":5,"soft_fail":false,"target_va":5383308144,"va":5374402388,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df4230
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_update.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8933907","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374401904,"raw_hex":"e813528800","size":5,"soft_fail":false,"target_va":5383340592,"va":5374406680,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=10
//     evidence={"assembly":"\tcallq\t-3097162","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374401904,"raw_hex":"e8b6bdd0ff","size":5,"soft_fail":false,"target_va":5371306912,"va":5374404069,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28571402","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374401904,"raw_hex":"e80af7b301","size":5,"soft_fail":false,"target_va":5402978688,"va":5374407281,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t28571582","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374401904,"raw_hex":"e8bef7b301","size":5,"soft_fail":false,"target_va":5402978880,"va":5374407293,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28571800","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374401904,"raw_hex":"e898f8b301","size":5,"soft_fail":false,"target_va":5402979152,"va":5374407347,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28572072","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374401904,"raw_hex":"e8a8f9b301","size":5,"soft_fail":false,"target_va":5402979344,"va":5374407267,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-12723","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374401904,"raw_hex":"e84dceffff","size":5,"soft_fail":false,"target_va":5374396768,"va":5374409486,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28577683","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374401904,"raw_hex":"e8930fb401","size":5,"soft_fail":false,"target_va":5402980128,"va":5374402440,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-5062","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374401904,"raw_hex":"e83aecffff","size":5,"soft_fail":false,"target_va":5374396960,"va":5374402017,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t28578557","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374401904,"raw_hex":"e8fd12b401","size":5,"soft_fail":false,"target_va":5402980656,"va":5374402094,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1954304","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374401904,"raw_hex":"e800d21d00","size":5,"soft_fail":false,"target_va":5376356912,"va":5374402603,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1958187","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374401904,"raw_hex":"e82be11d00","size":5,"soft_fail":false,"target_va":5376360560,"va":5374402368,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1954153","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374401904,"raw_hex":"e869d11d00","size":5,"soft_fail":false,"target_va":5376356144,"va":5374401986,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tjmp\t-9963","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5374401904,"raw_hex":"e915d9ffff","size":5,"soft_fail":false,"target_va":5374396768,"va":5374406726,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":410,"record_offset":28252,"parent_record_offset":21876,"ancestor_record_offset":21876,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=11
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":410,"record_offset":26144,"parent_record_offset":21876,"ancestor_record_offset":21876,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventReader::read
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":410,"record_offset":24776,"parent_record_offset":21876,"ancestor_record_offset":21876,"inlinee":6366,"source_line":468,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventWriter::send
//     owner=git_dependency_definition; count=9
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":410,"record_offset":34256,"parent_record_offset":21876,"ancestor_record_offset":21876,"inlinee":7450,"source_line":624,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::impl$20::last
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":410,"record_offset":25068,"parent_record_offset":21876,"ancestor_record_offset":21876,"inlinee":7511,"source_line":782,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":410,"record_offset":29540,"parent_record_offset":21876,"ancestor_record_offset":21876,"inlinee":4576,"source_line":263,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":410,"record_offset":26104,"parent_record_offset":21876,"ancestor_record_offset":21876,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":410,"record_offset":28340,"parent_record_offset":21876,"ancestor_record_offset":21876,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":410,"record_offset":26468,"parent_record_offset":21876,"ancestor_record_offset":21876,"inlinee":4773,"source_line":1127,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":410,"record_offset":35700,"parent_record_offset":21876,"ancestor_record_offset":21876,"inlinee":4779,"source_line":953,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":410,"record_offset":28368,"parent_record_offset":21876,"ancestor_record_offset":21876,"inlinee":4556,"source_line":2154,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":410,"record_offset":26200,"parent_record_offset":21876,"ancestor_record_offset":21876,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::first
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":410,"record_offset":26072,"parent_record_offset":21876,"ancestor_record_offset":21876,"inlinee":12681,"source_line":176,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_direct] country_core::input::action_set::DigitalActionState::just_pressed
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/input/action_set.rs
//     evidence={"module":410,"record_offset":34420,"parent_record_offset":21876,"ancestor_record_offset":21876,"inlinee":21894,"source_line":148,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_direct] country_core::input::action_set::DigitalActionState::just_released
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/input/action_set.rs
//     evidence={"module":410,"record_offset":28220,"parent_record_offset":21876,"ancestor_record_offset":21876,"inlinee":21920,"source_line":152,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::resources::ui_state::impl$10::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":410,"record_offset":34856,"parent_record_offset":21876,"ancestor_record_offset":21876,"inlinee":22151,"source_line":10,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::resources::widget_params::WidgetVolumeShape::get_origin
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/widget_params.rs
//     evidence={"module":410,"record_offset":28748,"parent_record_offset":21876,"ancestor_record_offset":21876,"inlinee":23533,"source_line":302,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::ui_update::enter_hover
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/ui_update.rs
//     evidence={"module":410,"record_offset":26524,"parent_record_offset":21876,"ancestor_record_offset":21876,"inlinee":23530,"source_line":261,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::ui_update::get_ui_volume
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/ui_update.rs
//     evidence={"module":410,"record_offset":29596,"parent_record_offset":21876,"ancestor_record_offset":21876,"inlinee":23531,"source_line":12,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":410,"record_offset":23708,"parent_record_offset":21876,"ancestor_record_offset":21876,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":410,"record_offset":22100,"parent_record_offset":21876,"ancestor_record_offset":21876,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":410,"record_offset":22688,"parent_record_offset":21876,"ancestor_record_offset":21876,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":410,"record_offset":37488,"parent_record_offset":21876,"ancestor_record_offset":21876,"inlinee":17320,"source_line":710,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":410,"record_offset":30820,"parent_record_offset":21876,"ancestor_record_offset":21876,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":410,"record_offset":27992,"parent_record_offset":27960,"ancestor_record_offset":21876,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":410,"record_offset":26272,"parent_record_offset":26248,"ancestor_record_offset":21876,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":410,"record_offset":27680,"parent_record_offset":26576,"ancestor_record_offset":21876,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":410,"record_offset":26248,"parent_record_offset":26224,"ancestor_record_offset":21876,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":410,"record_offset":27960,"parent_record_offset":27920,"ancestor_record_offset":21876,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":410,"record_offset":27920,"parent_record_offset":27884,"ancestor_record_offset":21876,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":410,"record_offset":28792,"parent_record_offset":28748,"ancestor_record_offset":21876,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=21
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":410,"record_offset":26168,"parent_record_offset":26144,"ancestor_record_offset":21876,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=10
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":410,"record_offset":27824,"parent_record_offset":27800,"ancestor_record_offset":21876,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::EventIteratorWithId::new
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":410,"record_offset":24888,"parent_record_offset":24856,"ancestor_record_offset":21876,"inlinee":6371,"source_line":810,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::EventWriter::send
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":410,"record_offset":27800,"parent_record_offset":26524,"ancestor_record_offset":21876,"inlinee":7450,"source_line":624,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::send
//     owner=git_dependency_definition; count=12
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":410,"record_offset":27884,"parent_record_offset":27800,"ancestor_record_offset":21876,"inlinee":7451,"source_line":231,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::ManualEventReader::read_with_id
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":410,"record_offset":24856,"parent_record_offset":24808,"ancestor_record_offset":21876,"inlinee":6369,"source_line":722,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::impl$23::last
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":410,"record_offset":25100,"parent_record_offset":25068,"ancestor_record_offset":21876,"inlinee":7512,"source_line":867,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs
//     evidence={"module":410,"record_offset":24808,"parent_record_offset":24776,"ancestor_record_offset":21876,"inlinee":7509,"source_line":805,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":410,"record_offset":24128,"parent_record_offset":24100,"ancestor_record_offset":21876,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":410,"record_offset":24180,"parent_record_offset":24156,"ancestor_record_offset":21876,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":410,"record_offset":24156,"parent_record_offset":24128,"ancestor_record_offset":21876,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":410,"record_offset":24068,"parent_record_offset":24036,"ancestor_record_offset":21876,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":410,"record_offset":24100,"parent_record_offset":24068,"ancestor_record_offset":21876,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":410,"record_offset":24272,"parent_record_offset":24100,"ancestor_record_offset":21876,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":410,"record_offset":24384,"parent_record_offset":24360,"ancestor_record_offset":21876,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":410,"record_offset":27352,"parent_record_offset":27328,"ancestor_record_offset":21876,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":410,"record_offset":26904,"parent_record_offset":26880,"ancestor_record_offset":21876,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":410,"record_offset":26988,"parent_record_offset":26964,"ancestor_record_offset":21876,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":410,"record_offset":26928,"parent_record_offset":26904,"ancestor_record_offset":21876,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::last
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":410,"record_offset":25132,"parent_record_offset":25100,"ancestor_record_offset":21876,"inlinee":8099,"source_line":143,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":410,"record_offset":24204,"parent_record_offset":24180,"ancestor_record_offset":21876,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":410,"record_offset":24932,"parent_record_offset":24888,"ancestor_record_offset":21876,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":410,"record_offset":22316,"parent_record_offset":22220,"ancestor_record_offset":21876,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":410,"record_offset":27100,"parent_record_offset":27076,"ancestor_record_offset":21876,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":410,"record_offset":35372,"parent_record_offset":35344,"ancestor_record_offset":21876,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":410,"record_offset":25160,"parent_record_offset":25132,"ancestor_record_offset":21876,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":410,"record_offset":23960,"parent_record_offset":23932,"ancestor_record_offset":21876,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":410,"record_offset":23288,"parent_record_offset":22828,"ancestor_record_offset":21876,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":410,"record_offset":24008,"parent_record_offset":23828,"ancestor_record_offset":21876,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":410,"record_offset":26032,"parent_record_offset":25948,"ancestor_record_offset":21876,"inlinee":8100,"source_line":1485,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":410,"record_offset":27768,"parent_record_offset":26576,"ancestor_record_offset":21876,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_default
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":410,"record_offset":24996,"parent_record_offset":24888,"ancestor_record_offset":21876,"inlinee":4156,"source_line":1003,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$15::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":410,"record_offset":28392,"parent_record_offset":28368,"ancestor_record_offset":21876,"inlinee":4558,"source_line":2174,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$40::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":410,"record_offset":25220,"parent_record_offset":25100,"ancestor_record_offset":21876,"inlinee":5366,"source_line":2485,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":410,"record_offset":22636,"parent_record_offset":22608,"ancestor_record_offset":21876,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=27
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":410,"record_offset":24336,"parent_record_offset":24036,"ancestor_record_offset":21876,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":410,"record_offset":25752,"parent_record_offset":25728,"ancestor_record_offset":21876,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":410,"record_offset":27264,"parent_record_offset":27240,"ancestor_record_offset":21876,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":410,"record_offset":27240,"parent_record_offset":27216,"ancestor_record_offset":21876,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":410,"record_offset":26304,"parent_record_offset":26272,"ancestor_record_offset":21876,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":410,"record_offset":23064,"parent_record_offset":23040,"ancestor_record_offset":21876,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":410,"record_offset":23040,"parent_record_offset":23016,"ancestor_record_offset":21876,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":410,"record_offset":23248,"parent_record_offset":23216,"ancestor_record_offset":21876,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":410,"record_offset":24228,"parent_record_offset":24204,"ancestor_record_offset":21876,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":410,"record_offset":24736,"parent_record_offset":23788,"ancestor_record_offset":21876,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":410,"record_offset":22252,"parent_record_offset":22220,"ancestor_record_offset":21876,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":410,"record_offset":25700,"parent_record_offset":25356,"ancestor_record_offset":21876,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":410,"record_offset":28820,"parent_record_offset":28792,"ancestor_record_offset":21876,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":410,"record_offset":22280,"parent_record_offset":22252,"ancestor_record_offset":21876,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":410,"record_offset":28848,"parent_record_offset":28820,"ancestor_record_offset":21876,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":410,"record_offset":25560,"parent_record_offset":25536,"ancestor_record_offset":21876,"inlinee":7968,"source_line":353,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":410,"record_offset":25536,"parent_record_offset":25488,"ancestor_record_offset":21876,"inlinee":8098,"source_line":479,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":410,"record_offset":25728,"parent_record_offset":25700,"ancestor_record_offset":21876,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":410,"record_offset":23016,"parent_record_offset":22992,"ancestor_record_offset":21876,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":410,"record_offset":23344,"parent_record_offset":23320,"ancestor_record_offset":21876,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":410,"record_offset":22992,"parent_record_offset":22884,"ancestor_record_offset":21876,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":410,"record_offset":22220,"parent_record_offset":22192,"ancestor_record_offset":21876,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":410,"record_offset":22724,"parent_record_offset":22688,"ancestor_record_offset":21876,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":410,"record_offset":22856,"parent_record_offset":22828,"ancestor_record_offset":21876,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":410,"record_offset":22828,"parent_record_offset":22796,"ancestor_record_offset":21876,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":410,"record_offset":22772,"parent_record_offset":22748,"ancestor_record_offset":21876,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":410,"record_offset":22796,"parent_record_offset":22772,"ancestor_record_offset":21876,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":410,"record_offset":22748,"parent_record_offset":22724,"ancestor_record_offset":21876,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":410,"record_offset":23320,"parent_record_offset":22796,"ancestor_record_offset":21876,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":410,"record_offset":22424,"parent_record_offset":22388,"ancestor_record_offset":21876,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":410,"record_offset":22584,"parent_record_offset":22424,"ancestor_record_offset":21876,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":410,"record_offset":22388,"parent_record_offset":22360,"ancestor_record_offset":21876,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":410,"record_offset":22136,"parent_record_offset":22100,"ancestor_record_offset":21876,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":410,"record_offset":22608,"parent_record_offset":22584,"ancestor_record_offset":21876,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":410,"record_offset":22192,"parent_record_offset":22160,"ancestor_record_offset":21876,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":410,"record_offset":22160,"parent_record_offset":22136,"ancestor_record_offset":21876,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":410,"record_offset":22884,"parent_record_offset":22856,"ancestor_record_offset":21876,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":410,"record_offset":23132,"parent_record_offset":22884,"ancestor_record_offset":21876,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":410,"record_offset":23104,"parent_record_offset":22884,"ancestor_record_offset":21876,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":410,"record_offset":22936,"parent_record_offset":22884,"ancestor_record_offset":21876,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":410,"record_offset":27708,"parent_record_offset":27680,"ancestor_record_offset":21876,"inlinee":5317,"source_line":2665,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":410,"record_offset":28124,"parent_record_offset":28088,"ancestor_record_offset":21876,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":410,"record_offset":27732,"parent_record_offset":27708,"ancestor_record_offset":21876,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":410,"record_offset":26332,"parent_record_offset":26272,"ancestor_record_offset":21876,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":410,"record_offset":26428,"parent_record_offset":26396,"ancestor_record_offset":21876,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::ui_state::impl$10::eq
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":410,"record_offset":27376,"parent_record_offset":27352,"ancestor_record_offset":21876,"inlinee":22151,"source_line":10,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::ui_update::get_ui_volume
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/ui_update.rs
//     evidence={"module":410,"record_offset":26576,"parent_record_offset":26524,"ancestor_record_offset":21876,"inlinee":23531,"source_line":12,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::ui_update::ui_update::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/ui_update.rs
//     evidence={"module":410,"record_offset":26492,"parent_record_offset":26468,"ancestor_record_offset":21876,"inlinee":23528,"source_line":36,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::ui_update::ui_update::closure$2
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/ui_update.rs
//     evidence={"module":410,"record_offset":35320,"parent_record_offset":35296,"ancestor_record_offset":21876,"inlinee":23534,"source_line":210,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":410,"record_offset":35344,"parent_record_offset":35320,"ancestor_record_offset":21876,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":410,"record_offset":27328,"parent_record_offset":27304,"ancestor_record_offset":21876,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::clear
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":410,"record_offset":37512,"parent_record_offset":37488,"ancestor_record_offset":21876,"inlinee":14997,"source_line":1011,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":410,"record_offset":26644,"parent_record_offset":26612,"ancestor_record_offset":21876,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":410,"record_offset":26676,"parent_record_offset":26644,"ancestor_record_offset":21876,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":410,"record_offset":27304,"parent_record_offset":27168,"ancestor_record_offset":21876,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":410,"record_offset":26716,"parent_record_offset":26676,"ancestor_record_offset":21876,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":410,"record_offset":27216,"parent_record_offset":27192,"ancestor_record_offset":21876,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":410,"record_offset":27192,"parent_record_offset":27168,"ancestor_record_offset":21876,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":410,"record_offset":27572,"parent_record_offset":27548,"ancestor_record_offset":21876,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":410,"record_offset":27512,"parent_record_offset":26808,"ancestor_record_offset":21876,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":410,"record_offset":27548,"parent_record_offset":26776,"ancestor_record_offset":21876,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":410,"record_offset":26776,"parent_record_offset":26744,"ancestor_record_offset":21876,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":410,"record_offset":26744,"parent_record_offset":26676,"ancestor_record_offset":21876,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":410,"record_offset":26808,"parent_record_offset":26776,"ancestor_record_offset":21876,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":410,"record_offset":27048,"parent_record_offset":27020,"ancestor_record_offset":21876,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":410,"record_offset":27076,"parent_record_offset":27048,"ancestor_record_offset":21876,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":410,"record_offset":27136,"parent_record_offset":27020,"ancestor_record_offset":21876,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":410,"record_offset":27020,"parent_record_offset":26808,"ancestor_record_offset":21876,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":410,"record_offset":26852,"parent_record_offset":26808,"ancestor_record_offset":21876,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":410,"record_offset":27168,"parent_record_offset":26808,"ancestor_record_offset":21876,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":410,"record_offset":26880,"parent_record_offset":26808,"ancestor_record_offset":21876,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=10
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":410,"record_offset":26964,"parent_record_offset":26808,"ancestor_record_offset":21876,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":410,"record_offset":27420,"parent_record_offset":26808,"ancestor_record_offset":21876,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":410,"record_offset":23748,"parent_record_offset":23708,"ancestor_record_offset":21876,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":410,"record_offset":23872,"parent_record_offset":23828,"ancestor_record_offset":21876,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":410,"record_offset":24036,"parent_record_offset":23828,"ancestor_record_offset":21876,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":410,"record_offset":24308,"parent_record_offset":24036,"ancestor_record_offset":21876,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":410,"record_offset":26612,"parent_record_offset":26576,"ancestor_record_offset":21876,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":410,"record_offset":23900,"parent_record_offset":23872,"ancestor_record_offset":21876,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":410,"record_offset":23932,"parent_record_offset":23900,"ancestor_record_offset":21876,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":410,"record_offset":23828,"parent_record_offset":23788,"ancestor_record_offset":21876,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":410,"record_offset":23788,"parent_record_offset":23748,"ancestor_record_offset":21876,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui::ui_update::get_ui_volume
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 12
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5BEB
// 首条位置: {"module":410,"inlinee":23531}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":410,"record_offset":27680,"parent_record_offset":26576,"ancestor_record_offset":26576,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":410,"record_offset":27768,"parent_record_offset":26576,"ancestor_record_offset":26576,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":410,"record_offset":26612,"parent_record_offset":26576,"ancestor_record_offset":26576,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":410,"record_offset":27352,"parent_record_offset":27328,"ancestor_record_offset":26576,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":410,"record_offset":26904,"parent_record_offset":26880,"ancestor_record_offset":26576,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":410,"record_offset":26988,"parent_record_offset":26964,"ancestor_record_offset":26576,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":410,"record_offset":26928,"parent_record_offset":26904,"ancestor_record_offset":26576,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":410,"record_offset":27100,"parent_record_offset":27076,"ancestor_record_offset":26576,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":410,"record_offset":27264,"parent_record_offset":27240,"ancestor_record_offset":26576,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":410,"record_offset":27240,"parent_record_offset":27216,"ancestor_record_offset":26576,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":410,"record_offset":27708,"parent_record_offset":27680,"ancestor_record_offset":26576,"inlinee":5317,"source_line":2665,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":410,"record_offset":27732,"parent_record_offset":27708,"ancestor_record_offset":26576,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::ui_state::impl$10::eq
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":410,"record_offset":27376,"parent_record_offset":27352,"ancestor_record_offset":26576,"inlinee":22151,"source_line":10,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":410,"record_offset":27328,"parent_record_offset":27304,"ancestor_record_offset":26576,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":410,"record_offset":26644,"parent_record_offset":26612,"ancestor_record_offset":26576,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":410,"record_offset":26676,"parent_record_offset":26644,"ancestor_record_offset":26576,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":410,"record_offset":27304,"parent_record_offset":27168,"ancestor_record_offset":26576,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":410,"record_offset":26716,"parent_record_offset":26676,"ancestor_record_offset":26576,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":410,"record_offset":27216,"parent_record_offset":27192,"ancestor_record_offset":26576,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":410,"record_offset":27192,"parent_record_offset":27168,"ancestor_record_offset":26576,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":410,"record_offset":27572,"parent_record_offset":27548,"ancestor_record_offset":26576,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":410,"record_offset":27512,"parent_record_offset":26808,"ancestor_record_offset":26576,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":410,"record_offset":27548,"parent_record_offset":26776,"ancestor_record_offset":26576,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":410,"record_offset":26776,"parent_record_offset":26744,"ancestor_record_offset":26576,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":410,"record_offset":26744,"parent_record_offset":26676,"ancestor_record_offset":26576,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":410,"record_offset":26808,"parent_record_offset":26776,"ancestor_record_offset":26576,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":410,"record_offset":27048,"parent_record_offset":27020,"ancestor_record_offset":26576,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":410,"record_offset":27076,"parent_record_offset":27048,"ancestor_record_offset":26576,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":410,"record_offset":27136,"parent_record_offset":27020,"ancestor_record_offset":26576,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":410,"record_offset":27020,"parent_record_offset":26808,"ancestor_record_offset":26576,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":410,"record_offset":26852,"parent_record_offset":26808,"ancestor_record_offset":26576,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":410,"record_offset":27168,"parent_record_offset":26808,"ancestor_record_offset":26576,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":410,"record_offset":26880,"parent_record_offset":26808,"ancestor_record_offset":26576,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=8
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":410,"record_offset":26964,"parent_record_offset":26808,"ancestor_record_offset":26576,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":410,"record_offset":27420,"parent_record_offset":26808,"ancestor_record_offset":26576,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui::ui_update::ui_update::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 36
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5BE8
// 首条位置: {"module":410,"inlinee":23528}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui::ui_update::ui_update::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 210
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5BEE
// 首条位置: {"module":410,"inlinee":23534}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":410,"record_offset":35344,"parent_record_offset":35320,"ancestor_record_offset":35320,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":410,"record_offset":35372,"parent_record_offset":35344,"ancestor_record_offset":35320,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui::ui_update::enter_hover
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 261
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5BEA
// 首条位置: {"module":410,"inlinee":23530}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::event::EventWriter::send
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":410,"record_offset":27800,"parent_record_offset":26524,"ancestor_record_offset":26524,"inlinee":7450,"source_line":624,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":410,"record_offset":28064,"parent_record_offset":26524,"ancestor_record_offset":26524,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::ui_update::get_ui_volume
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/ui_update.rs
//     evidence={"module":410,"record_offset":26576,"parent_record_offset":26524,"ancestor_record_offset":26524,"inlinee":23531,"source_line":12,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":410,"record_offset":27992,"parent_record_offset":27960,"ancestor_record_offset":26524,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":410,"record_offset":27680,"parent_record_offset":26576,"ancestor_record_offset":26524,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":410,"record_offset":28088,"parent_record_offset":28064,"ancestor_record_offset":26524,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":410,"record_offset":27960,"parent_record_offset":27920,"ancestor_record_offset":26524,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":410,"record_offset":27920,"parent_record_offset":27884,"ancestor_record_offset":26524,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":410,"record_offset":27852,"parent_record_offset":27824,"ancestor_record_offset":26524,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":410,"record_offset":27824,"parent_record_offset":27800,"ancestor_record_offset":26524,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::send
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":410,"record_offset":27884,"parent_record_offset":27800,"ancestor_record_offset":26524,"inlinee":7451,"source_line":231,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":410,"record_offset":27352,"parent_record_offset":27328,"ancestor_record_offset":26524,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":410,"record_offset":26904,"parent_record_offset":26880,"ancestor_record_offset":26524,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":410,"record_offset":26988,"parent_record_offset":26964,"ancestor_record_offset":26524,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":410,"record_offset":26928,"parent_record_offset":26904,"ancestor_record_offset":26524,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":410,"record_offset":27100,"parent_record_offset":27076,"ancestor_record_offset":26524,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":410,"record_offset":27768,"parent_record_offset":26576,"ancestor_record_offset":26524,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":410,"record_offset":27264,"parent_record_offset":27240,"ancestor_record_offset":26524,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":410,"record_offset":27240,"parent_record_offset":27216,"ancestor_record_offset":26524,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":410,"record_offset":28024,"parent_record_offset":27920,"ancestor_record_offset":26524,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":410,"record_offset":27708,"parent_record_offset":27680,"ancestor_record_offset":26524,"inlinee":5317,"source_line":2665,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":410,"record_offset":28124,"parent_record_offset":28088,"ancestor_record_offset":26524,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":410,"record_offset":27732,"parent_record_offset":27708,"ancestor_record_offset":26524,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":410,"record_offset":28148,"parent_record_offset":28124,"ancestor_record_offset":26524,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":410,"record_offset":28180,"parent_record_offset":28088,"ancestor_record_offset":26524,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::ui_state::impl$10::eq
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":410,"record_offset":27376,"parent_record_offset":27352,"ancestor_record_offset":26524,"inlinee":22151,"source_line":10,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":410,"record_offset":27328,"parent_record_offset":27304,"ancestor_record_offset":26524,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":410,"record_offset":26644,"parent_record_offset":26612,"ancestor_record_offset":26524,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":410,"record_offset":26676,"parent_record_offset":26644,"ancestor_record_offset":26524,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":410,"record_offset":27304,"parent_record_offset":27168,"ancestor_record_offset":26524,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":410,"record_offset":26716,"parent_record_offset":26676,"ancestor_record_offset":26524,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":410,"record_offset":27216,"parent_record_offset":27192,"ancestor_record_offset":26524,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":410,"record_offset":27192,"parent_record_offset":27168,"ancestor_record_offset":26524,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":410,"record_offset":27572,"parent_record_offset":27548,"ancestor_record_offset":26524,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":410,"record_offset":27512,"parent_record_offset":26808,"ancestor_record_offset":26524,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":410,"record_offset":27548,"parent_record_offset":26776,"ancestor_record_offset":26524,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":410,"record_offset":26776,"parent_record_offset":26744,"ancestor_record_offset":26524,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":410,"record_offset":26744,"parent_record_offset":26676,"ancestor_record_offset":26524,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":410,"record_offset":26808,"parent_record_offset":26776,"ancestor_record_offset":26524,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":410,"record_offset":27048,"parent_record_offset":27020,"ancestor_record_offset":26524,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":410,"record_offset":27076,"parent_record_offset":27048,"ancestor_record_offset":26524,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":410,"record_offset":27136,"parent_record_offset":27020,"ancestor_record_offset":26524,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":410,"record_offset":27020,"parent_record_offset":26808,"ancestor_record_offset":26524,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":410,"record_offset":26852,"parent_record_offset":26808,"ancestor_record_offset":26524,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":410,"record_offset":27168,"parent_record_offset":26808,"ancestor_record_offset":26524,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":410,"record_offset":26880,"parent_record_offset":26808,"ancestor_record_offset":26524,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":410,"record_offset":26964,"parent_record_offset":26808,"ancestor_record_offset":26524,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":410,"record_offset":27420,"parent_record_offset":26808,"ancestor_record_offset":26524,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":410,"record_offset":26612,"parent_record_offset":26576,"ancestor_record_offset":26524,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/00410.jsonl.gz"}
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
// 完整结构及原始类型记录: ui_update.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: ui_update.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: ui_update.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::event::EventReader<country_core::resources::window::WindowFocusEvent>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventWriter<country_core::systems::brush_preview::OverwriteBrushPreview>,bevy_ecs::event::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "bevy_ecs::event::EventReader<country_core::resources::window::WindowFocusEvent>", "bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>", "bevy_ecs::event::EventWriter<country_core::systems::brush_preview::OverwriteBrushPreview>", "bevy_ecs::event::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::window::WindowFocusEvent>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::brush_preview::OverwriteBrushPreview>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >

// ------------------------------------------------------------------
// 函数: country_core::systems::ui::ui_update::enter_hover
// 信息 ID: afd495de2d1af3abd45f8db6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::ui::ui_update::ui_update; module=410, inline_record=26524; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui::ui_update::ui_update; module=410, inline_record=31912; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui::ui_update::ui_update; module=410, inline_record=37544; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ui::ui_update::get_ui_volume
// 信息 ID: d4f004d7b181a702f37dabad
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::ui::ui_update::ui_update; module=410, inline_record=26576; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui::ui_update::ui_update; module=410, inline_record=29596; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui::ui_update::ui_update; module=410, inline_record=31964; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui::ui_update::ui_update; module=410, inline_record=37592; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ui::ui_update::ui_update
// 信息 ID: 94ef3b37c4bfb7401fb67d22
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 6afd8a1a301f5d1cb5809105: void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::event::EventReader<country_core::resources::window::WindowFocusEvent>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventWriter<country_core::systems::brush_preview::OverwriteBrushPreview>,bevy_ecs::event::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::event::EventReader<country_core::resources::window::WindowFocusEvent>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventWriter<country_core::systems::brush_preview::OverwriteBrushPreview>,bevy_ecs::event::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::event::EventReader<country_core::resources::window::WindowFocusEvent>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventWriter<country_core::systems::brush_preview::OverwriteBrushPreview>,bevy_ecs::event::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >)> > @ 0xe6616a -> 0x56dd70 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::event::EventReader<country_core::resources::window::WindowFocusEvent>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventWriter<country_core::systems::brush_preview::OverwriteBrushPreview>,bevy_ecs::event::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::event::EventReader<country_core::resources::window::WindowFocusEvent>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventWriter<country_core::systems::brush_preview::OverwriteBrushPreview>,bevy_ecs::event::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >)> @ 0x1140634 -> 0x56dd70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ui::ui_update::ui_update::closure$0
// 信息 ID: d76146207462e3b624834d76
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::ui::ui_update::ui_update", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::ui::ui_update::ui_update; module=410, inline_record=26492; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ui::ui_update::ui_update::closure$2
// 信息 ID: 7cb42ba58f88602c81951e3e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "country_core::systems::ui::ui_update::ui_update", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::ui::ui_update::ui_update; module=410, inline_record=35320; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
