// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/decorator/src/decorator_visual/gothic_balcony_door.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: b1ffcd474276df569557013324800209c65b8e70c92e3e6148be7f0a967b2953
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: gothic_balcony_door.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 3850
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
// use system_decorator; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::gothic_balcony_door::generate_gothic_balcony_doors
// 记录形式: compiled_procedure; instances=1
// 源码行: 25
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3850,"record_offset":3608,"rva":29659792,"va":5398368912}
// 编译实例 RVAs: 0x1c49290
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: gothic_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28757349","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398368912,"raw_hex":"e89b3249fe","size":5,"soft_fail":false,"target_va":5369613520,"va":5398370864,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14010da30
//     owner=unresolved_namespace; count=1
//     target_aliases=12 candidates; complete list: gothic_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28560313","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398368912,"raw_hex":"e847344cfe","size":5,"soft_fail":false,"target_va":5369813552,"va":5398373860,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402ce6a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: gothic_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26723460","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398368912,"raw_hex":"e87c3b68fe","size":5,"soft_fail":false,"target_va":5371651744,"va":5398375199,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: gothic_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25622977","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398368912,"raw_hex":"e83f0679fe","size":5,"soft_fail":false,"target_va":5372749696,"va":5398372668,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: gothic_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25623527","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398368912,"raw_hex":"e8190479fe","size":5,"soft_fail":false,"target_va":5372749712,"va":5398373234,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14051aa00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gothic_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24312847","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398368912,"raw_hex":"e8f1038dfe","size":5,"soft_fail":false,"target_va":5374061056,"va":5398373898,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140593250
//     owner=unresolved_namespace; count=3
//     target_aliases=15 candidates; complete list: gothic_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23816411","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398368912,"raw_hex":"e8259794fe","size":5,"soft_fail":false,"target_va":5374554704,"va":5398371110,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b9c30
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: gothic_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23656855","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398368912,"raw_hex":"e8690697fe","size":5,"soft_fail":false,"target_va":5374712880,"va":5398369730,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405bab80
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: gothic_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23653933","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398368912,"raw_hex":"e8d31197fe","size":5,"soft_fail":false,"target_va":5374716800,"va":5398370728,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140633130
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: gothic_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23161360","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398368912,"raw_hex":"e8f0959efe","size":5,"soft_fail":false,"target_va":5375209776,"va":5398371131,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140633eb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gothic_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23156474","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398368912,"raw_hex":"e806a99efe","size":5,"soft_fail":false,"target_va":5375213232,"va":5398369701,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da760
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gothic_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21427491","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398368912,"raw_hex":"e8dd0ab9fe","size":5,"soft_fail":false,"target_va":5376943968,"va":5398371454,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c96d80
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: gothic_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16461676","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398368912,"raw_hex":"e894d004ff","size":5,"soft_fail":false,"target_va":5381909888,"va":5398371559,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d06720
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gothic_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16004155","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398368912,"raw_hex":"e8c5cb0bff","size":5,"soft_fail":false,"target_va":5382367008,"va":5398371158,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t4603446","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398368912,"raw_hex":"e8363e4600","size":5,"soft_fail":false,"target_va":5402978144,"va":5398374693,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t-27067517","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398368912,"raw_hex":"e883fb62fe","size":5,"soft_fail":false,"target_va":5371305408,"va":5398372920,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4603942","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398368912,"raw_hex":"e826404600","size":5,"soft_fail":false,"target_va":5402978688,"va":5398374741,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4604489","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398368912,"raw_hex":"e849424600","size":5,"soft_fail":false,"target_va":5402978912,"va":5398374418,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t4604700","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398368912,"raw_hex":"e81c434600","size":5,"soft_fail":false,"target_va":5402979152,"va":5398374447,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t4604670","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398368912,"raw_hex":"e8fe424600","size":5,"soft_fail":false,"target_va":5402978944,"va":5398374269,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-6516","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398368912,"raw_hex":"e88ce6ffff","size":5,"soft_fail":false,"target_va":5398367552,"va":5398374063,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4610639","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398368912,"raw_hex":"e84f5a4600","size":5,"soft_fail":false,"target_va":5402980128,"va":5398369484,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1321","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398368912,"raw_hex":"e8d7faffff","size":5,"soft_fail":false,"target_va":5398367744,"va":5398369060,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t4611514","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398368912,"raw_hex":"e8ba5d4600","size":5,"soft_fail":false,"target_va":5402980656,"va":5398369137,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorInfo::dst_as_wall
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t-23800083","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398368912,"raw_hex":"e8edd694fe","size":5,"soft_fail":false,"target_va":5374570992,"va":5398371070,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::subtype
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-23158000","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398368912,"raw_hex":"e810a39efe","size":5,"soft_fail":false,"target_va":5375212688,"va":5398370683,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-22012747","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398368912,"raw_hex":"e8b51cb0fe","size":5,"soft_fail":false,"target_va":5376356912,"va":5398369654,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-22008857","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398368912,"raw_hex":"e8e72bb0fe","size":5,"soft_fail":false,"target_va":5376360560,"va":5398369412,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-22012901","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398368912,"raw_hex":"e81b1cb0fe","size":5,"soft_fail":false,"target_va":5376356144,"va":5398369040,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::fast_local::Key$LT$T$GT$::try_initialize::hcc99216871ec738f (.llvm.16000913382889057913)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-16079629","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398368912,"raw_hex":"e8f3a40aff","size":5,"soft_fail":false,"target_va":5382289808,"va":5398369432,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::blueprint::DecoratorBlueprint::set_colliders
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-4881812","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398368912,"raw_hex":"e86c82b5ff","size":5,"soft_fail":false,"target_va":5393490672,"va":5398372479,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::blueprint::DecoratorBlueprints::insert
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-4879288","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398368912,"raw_hex":"e8488cb5ff","size":5,"soft_fail":false,"target_va":5393491488,"va":5398370771,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::decorator_visual::gothic_wall_window::get_gothic_hat_bones
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-37103","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398368912,"raw_hex":"e8116fffff","size":5,"soft_fail":false,"target_va":5398334144,"va":5398371242,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle3d::extent
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20419381","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398368912,"raw_hex":"e8cb6cc8fe","size":5,"soft_fail":false,"target_va":5377952848,"va":5398372224,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::xz_projective_rotation
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21428102","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398368912,"raw_hex":"e87a08b9fe","size":5,"soft_fail":false,"target_va":5376943376,"va":5398371473,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3850,"record_offset":7720,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":3850,"record_offset":15132,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3850,"record_offset":8516,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":3850,"record_offset":8572,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":3850,"record_offset":6616,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":3850,"record_offset":6588,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3850,"record_offset":15164,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::color_id::impl$11::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/color_id.rs
//     evidence={"module":3850,"record_offset":8600,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":61513,"source_line":26,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::bbx_ls
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3850,"record_offset":9692,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":21725,"source_line":267,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::collision_mesh
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3850,"record_offset":10292,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":60285,"source_line":274,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::interaction_mesh
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3850,"record_offset":11752,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":60287,"source_line":285,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3850,"record_offset":6532,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":21526,"source_line":249,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":3850,"record_offset":13328,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":14676,"source_line":659,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3850,"record_offset":10076,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":21371,"source_line":450,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3850,"record_offset":10264,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":14690,"source_line":814,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3850,"record_offset":9884,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":21436,"source_line":742,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$11::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3850,"record_offset":13216,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":25315,"source_line":1069,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3850,"record_offset":14180,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3850,"record_offset":13272,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3850,"record_offset":5484,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3850,"record_offset":3896,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3850,"record_offset":4472,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$36::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3850,"record_offset":7360,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":8927,"source_line":2181,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$39::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3850,"record_offset":7860,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":8642,"source_line":2233,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3850,"record_offset":8628,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":17187,"source_line":1342,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::blueprint::DecoratorBlueprint::set_outline
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/blueprint.rs
//     evidence={"module":3850,"record_offset":14268,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":60163,"source_line":214,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual::gothic_balcony_door::get_door_holes
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/gothic_balcony_door.rs
//     evidence={"module":3850,"record_offset":14388,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":111811,"source_line":143,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::mark_as_preview
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":3850,"record_offset":15416,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":60170,"source_line":72,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::new
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":3850,"record_offset":15388,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":60169,"source_line":49,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::width
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":3850,"record_offset":9856,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":21735,"source_line":70,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3850,"record_offset":14500,"parent_record_offset":14476,"ancestor_record_offset":3608,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3850,"record_offset":14524,"parent_record_offset":14500,"ancestor_record_offset":3608,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3850,"record_offset":15332,"parent_record_offset":15308,"ancestor_record_offset":3608,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3850,"record_offset":14444,"parent_record_offset":14388,"ancestor_record_offset":3608,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3850,"record_offset":14476,"parent_record_offset":14444,"ancestor_record_offset":3608,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3850,"record_offset":15308,"parent_record_offset":15284,"ancestor_record_offset":3608,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3850,"record_offset":7796,"parent_record_offset":7764,"ancestor_record_offset":3608,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3850,"record_offset":15284,"parent_record_offset":15260,"ancestor_record_offset":3608,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":3850,"record_offset":16100,"parent_record_offset":16064,"ancestor_record_offset":3608,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":3850,"record_offset":11572,"parent_record_offset":10328,"ancestor_record_offset":3608,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":3850,"record_offset":15688,"parent_record_offset":15664,"ancestor_record_offset":3608,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3850,"record_offset":7764,"parent_record_offset":7720,"ancestor_record_offset":3608,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":3850,"record_offset":15188,"parent_record_offset":15164,"ancestor_record_offset":3608,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":3850,"record_offset":15236,"parent_record_offset":15212,"ancestor_record_offset":3608,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3850,"record_offset":8540,"parent_record_offset":8516,"ancestor_record_offset":3608,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3850,"record_offset":5884,"parent_record_offset":5856,"ancestor_record_offset":3608,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3850,"record_offset":5936,"parent_record_offset":5912,"ancestor_record_offset":3608,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3850,"record_offset":5912,"parent_record_offset":5884,"ancestor_record_offset":3608,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3850,"record_offset":5824,"parent_record_offset":5796,"ancestor_record_offset":3608,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3850,"record_offset":5856,"parent_record_offset":5824,"ancestor_record_offset":3608,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3850,"record_offset":6028,"parent_record_offset":5856,"ancestor_record_offset":3608,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3850,"record_offset":6140,"parent_record_offset":6116,"ancestor_record_offset":3608,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3850,"record_offset":9472,"parent_record_offset":9448,"ancestor_record_offset":3608,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":3850,"record_offset":6556,"parent_record_offset":6532,"ancestor_record_offset":3608,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3850,"record_offset":6980,"parent_record_offset":6948,"ancestor_record_offset":3608,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3850,"record_offset":8932,"parent_record_offset":8908,"ancestor_record_offset":3608,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3850,"record_offset":7044,"parent_record_offset":7008,"ancestor_record_offset":3608,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3850,"record_offset":13852,"parent_record_offset":13436,"ancestor_record_offset":3608,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3850,"record_offset":13760,"parent_record_offset":13704,"ancestor_record_offset":3608,"inlinee":21747,"source_line":20,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3850,"record_offset":13532,"parent_record_offset":13508,"ancestor_record_offset":3608,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3850,"record_offset":13564,"parent_record_offset":13480,"ancestor_record_offset":3608,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3850,"record_offset":13624,"parent_record_offset":13436,"ancestor_record_offset":3608,"inlinee":14685,"source_line":54,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":3850,"record_offset":14356,"parent_record_offset":14268,"ancestor_record_offset":3608,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":3850,"record_offset":9812,"parent_record_offset":9788,"ancestor_record_offset":3608,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":3850,"record_offset":8956,"parent_record_offset":8932,"ancestor_record_offset":3608,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":3850,"record_offset":5960,"parent_record_offset":5936,"ancestor_record_offset":3608,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":3850,"record_offset":4108,"parent_record_offset":4012,"ancestor_record_offset":3608,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":3850,"record_offset":6876,"parent_record_offset":6852,"ancestor_record_offset":3608,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3850,"record_offset":10012,"parent_record_offset":9916,"ancestor_record_offset":3608,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3850,"record_offset":14208,"parent_record_offset":14180,"ancestor_record_offset":3608,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3850,"record_offset":9956,"parent_record_offset":9916,"ancestor_record_offset":3608,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":3850,"record_offset":7324,"parent_record_offset":7292,"ancestor_record_offset":3608,"inlinee":4428,"source_line":304,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3850,"record_offset":5720,"parent_record_offset":5692,"ancestor_record_offset":3608,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3850,"record_offset":14300,"parent_record_offset":14268,"ancestor_record_offset":3608,"inlinee":4710,"source_line":651,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3850,"record_offset":14324,"parent_record_offset":14300,"ancestor_record_offset":3608,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3850,"record_offset":5068,"parent_record_offset":4608,"ancestor_record_offset":3608,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3850,"record_offset":5768,"parent_record_offset":5592,"ancestor_record_offset":3608,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3850,"record_offset":9756,"parent_record_offset":9728,"ancestor_record_offset":3608,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3850,"record_offset":4420,"parent_record_offset":4392,"ancestor_record_offset":3608,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3850,"record_offset":6092,"parent_record_offset":5796,"ancestor_record_offset":3608,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3850,"record_offset":7128,"parent_record_offset":7104,"ancestor_record_offset":3608,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3850,"record_offset":7104,"parent_record_offset":7080,"ancestor_record_offset":3608,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3850,"record_offset":16132,"parent_record_offset":16100,"ancestor_record_offset":3608,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3850,"record_offset":4844,"parent_record_offset":4820,"ancestor_record_offset":3608,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3850,"record_offset":4820,"parent_record_offset":4796,"ancestor_record_offset":3608,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3850,"record_offset":5028,"parent_record_offset":4996,"ancestor_record_offset":3608,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3850,"record_offset":14552,"parent_record_offset":14524,"ancestor_record_offset":3608,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3850,"record_offset":5984,"parent_record_offset":5960,"ancestor_record_offset":3608,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3850,"record_offset":6492,"parent_record_offset":5556,"ancestor_record_offset":3608,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3850,"record_offset":4044,"parent_record_offset":4012,"ancestor_record_offset":3608,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3850,"record_offset":4072,"parent_record_offset":4044,"ancestor_record_offset":3608,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3850,"record_offset":4796,"parent_record_offset":4772,"ancestor_record_offset":3608,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3850,"record_offset":5124,"parent_record_offset":5100,"ancestor_record_offset":3608,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3850,"record_offset":4772,"parent_record_offset":4664,"ancestor_record_offset":3608,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":3850,"record_offset":4012,"parent_record_offset":3984,"ancestor_record_offset":3608,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":3850,"record_offset":4504,"parent_record_offset":4472,"ancestor_record_offset":3608,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":3850,"record_offset":4636,"parent_record_offset":4608,"ancestor_record_offset":3608,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":3850,"record_offset":4608,"parent_record_offset":4576,"ancestor_record_offset":3608,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3850,"record_offset":4552,"parent_record_offset":4528,"ancestor_record_offset":3608,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3850,"record_offset":4576,"parent_record_offset":4552,"ancestor_record_offset":3608,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3850,"record_offset":4528,"parent_record_offset":4504,"ancestor_record_offset":3608,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3850,"record_offset":5100,"parent_record_offset":4576,"ancestor_record_offset":3608,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3850,"record_offset":4208,"parent_record_offset":4176,"ancestor_record_offset":3608,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3850,"record_offset":4368,"parent_record_offset":4208,"ancestor_record_offset":3608,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3850,"record_offset":4176,"parent_record_offset":4152,"ancestor_record_offset":3608,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3850,"record_offset":3928,"parent_record_offset":3896,"ancestor_record_offset":3608,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3850,"record_offset":4392,"parent_record_offset":4368,"ancestor_record_offset":3608,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3850,"record_offset":3984,"parent_record_offset":3952,"ancestor_record_offset":3608,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3850,"record_offset":3952,"parent_record_offset":3928,"ancestor_record_offset":3608,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3850,"record_offset":4664,"parent_record_offset":4636,"ancestor_record_offset":3608,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3850,"record_offset":4912,"parent_record_offset":4664,"ancestor_record_offset":3608,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3850,"record_offset":4884,"parent_record_offset":4664,"ancestor_record_offset":3608,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3850,"record_offset":4716,"parent_record_offset":4664,"ancestor_record_offset":3608,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3850,"record_offset":11600,"parent_record_offset":11572,"ancestor_record_offset":3608,"inlinee":5317,"source_line":2665,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3850,"record_offset":15724,"parent_record_offset":15688,"ancestor_record_offset":3608,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3850,"record_offset":11624,"parent_record_offset":11600,"ancestor_record_offset":3608,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3850,"record_offset":15748,"parent_record_offset":15724,"ancestor_record_offset":3608,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3850,"record_offset":15780,"parent_record_offset":15688,"ancestor_record_offset":3608,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::collision_mesh
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3850,"record_offset":10328,"parent_record_offset":10292,"ancestor_record_offset":3608,"inlinee":60286,"source_line":225,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::get_collision_bbx_ls
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3850,"record_offset":9728,"parent_record_offset":9692,"ancestor_record_offset":3608,"inlinee":21727,"source_line":199,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::interaction_mesh
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3850,"record_offset":11792,"parent_record_offset":11752,"ancestor_record_offset":3608,"inlinee":60288,"source_line":234,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$21::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3850,"record_offset":11264,"parent_record_offset":11236,"ancestor_record_offset":3608,"inlinee":26328,"source_line":78,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$3::collision_mesh::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3850,"record_offset":11684,"parent_record_offset":11660,"ancestor_record_offset":3608,"inlinee":60292,"source_line":228,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$3::get_collision_bbx_ls::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3850,"record_offset":9788,"parent_record_offset":9756,"ancestor_record_offset":3608,"inlinee":21755,"source_line":202,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$3::interaction_mesh::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3850,"record_offset":13148,"parent_record_offset":13124,"ancestor_record_offset":3608,"inlinee":60290,"source_line":237,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":3850,"record_offset":9496,"parent_record_offset":9472,"ancestor_record_offset":3608,"inlinee":8043,"source_line":65,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":3850,"record_offset":13436,"parent_record_offset":13328,"ancestor_record_offset":3608,"inlinee":14681,"source_line":731,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":3850,"record_offset":13880,"parent_record_offset":13436,"ancestor_record_offset":3608,"inlinee":14682,"source_line":220,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":3850,"record_offset":13400,"parent_record_offset":13376,"ancestor_record_offset":3608,"inlinee":14680,"source_line":90,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$45::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":3850,"record_offset":13376,"parent_record_offset":13352,"ancestor_record_offset":3608,"inlinee":14678,"source_line":1358,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3850,"record_offset":10160,"parent_record_offset":10128,"ancestor_record_offset":3608,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3850,"record_offset":10128,"parent_record_offset":10100,"ancestor_record_offset":3608,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3850,"record_offset":10100,"parent_record_offset":10076,"ancestor_record_offset":3608,"inlinee":21372,"source_line":399,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3850,"record_offset":9916,"parent_record_offset":9884,"ancestor_record_offset":3608,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":3850,"record_offset":13508,"parent_record_offset":13480,"ancestor_record_offset":3608,"inlinee":21746,"source_line":42,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":3850,"record_offset":13480,"parent_record_offset":13436,"ancestor_record_offset":3608,"inlinee":21745,"source_line":66,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":3850,"record_offset":9448,"parent_record_offset":9424,"ancestor_record_offset":3608,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3850,"record_offset":8696,"parent_record_offset":8660,"ancestor_record_offset":3608,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3850,"record_offset":8720,"parent_record_offset":8696,"ancestor_record_offset":3608,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3850,"record_offset":7408,"parent_record_offset":7384,"ancestor_record_offset":3608,"inlinee":8929,"source_line":756,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3850,"record_offset":9424,"parent_record_offset":9200,"ancestor_record_offset":3608,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$60::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3850,"record_offset":6668,"parent_record_offset":6640,"ancestor_record_offset":3608,"inlinee":8644,"source_line":4727,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3850,"record_offset":8752,"parent_record_offset":8720,"ancestor_record_offset":3608,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3850,"record_offset":9336,"parent_record_offset":9312,"ancestor_record_offset":3608,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3850,"record_offset":9312,"parent_record_offset":9200,"ancestor_record_offset":3608,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3850,"record_offset":9252,"parent_record_offset":9228,"ancestor_record_offset":3608,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3850,"record_offset":7080,"parent_record_offset":6728,"ancestor_record_offset":3608,"inlinee":8358,"source_line":561,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3850,"record_offset":9632,"parent_record_offset":8828,"ancestor_record_offset":3608,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3850,"record_offset":7516,"parent_record_offset":7456,"ancestor_record_offset":3608,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3850,"record_offset":6728,"parent_record_offset":6696,"ancestor_record_offset":3608,"inlinee":8357,"source_line":3827,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3850,"record_offset":9228,"parent_record_offset":9200,"ancestor_record_offset":3608,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3850,"record_offset":8804,"parent_record_offset":8780,"ancestor_record_offset":3608,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3850,"record_offset":8780,"parent_record_offset":8720,"ancestor_record_offset":3608,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3850,"record_offset":7432,"parent_record_offset":7408,"ancestor_record_offset":3608,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3850,"record_offset":7488,"parent_record_offset":7456,"ancestor_record_offset":3608,"inlinee":6422,"source_line":2597,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3850,"record_offset":8828,"parent_record_offset":8804,"ancestor_record_offset":3608,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3850,"record_offset":7456,"parent_record_offset":7432,"ancestor_record_offset":3608,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3850,"record_offset":6816,"parent_record_offset":6780,"ancestor_record_offset":3608,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3850,"record_offset":6852,"parent_record_offset":6816,"ancestor_record_offset":3608,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3850,"record_offset":6912,"parent_record_offset":6780,"ancestor_record_offset":3608,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3850,"record_offset":6780,"parent_record_offset":6728,"ancestor_record_offset":3608,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3850,"record_offset":8880,"parent_record_offset":8828,"ancestor_record_offset":3608,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$28::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3850,"record_offset":6696,"parent_record_offset":6668,"ancestor_record_offset":3608,"inlinee":8355,"source_line":4127,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3850,"record_offset":9200,"parent_record_offset":8828,"ancestor_record_offset":3608,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3850,"record_offset":8908,"parent_record_offset":8828,"ancestor_record_offset":3608,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3850,"record_offset":6948,"parent_record_offset":6728,"ancestor_record_offset":3608,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3850,"record_offset":8992,"parent_record_offset":8828,"ancestor_record_offset":3608,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3850,"record_offset":9540,"parent_record_offset":8828,"ancestor_record_offset":3608,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3850,"record_offset":7008,"parent_record_offset":6948,"ancestor_record_offset":3608,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3850,"record_offset":5520,"parent_record_offset":5484,"ancestor_record_offset":3608,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":3850,"record_offset":5632,"parent_record_offset":5592,"ancestor_record_offset":3608,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3850,"record_offset":5796,"parent_record_offset":5592,"ancestor_record_offset":3608,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3850,"record_offset":6064,"parent_record_offset":5796,"ancestor_record_offset":3608,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3850,"record_offset":8660,"parent_record_offset":8628,"ancestor_record_offset":3608,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3850,"record_offset":7384,"parent_record_offset":7360,"ancestor_record_offset":3608,"inlinee":8928,"source_line":518,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$39::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3850,"record_offset":6640,"parent_record_offset":6616,"ancestor_record_offset":3608,"inlinee":8642,"source_line":2233,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":3850,"record_offset":10192,"parent_record_offset":10160,"ancestor_record_offset":3608,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":3850,"record_offset":5660,"parent_record_offset":5632,"ancestor_record_offset":3608,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":3850,"record_offset":5692,"parent_record_offset":5660,"ancestor_record_offset":3608,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":3850,"record_offset":5592,"parent_record_offset":5556,"ancestor_record_offset":3608,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":3850,"record_offset":5556,"parent_record_offset":5520,"ancestor_record_offset":3608,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::gothic_balcony_door::get_door_holes
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 143
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B4C3
// 首条位置: {"module":3850,"inlinee":111811}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3850,"record_offset":14444,"parent_record_offset":14388,"ancestor_record_offset":14388,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3850,"record_offset":14500,"parent_record_offset":14476,"ancestor_record_offset":14388,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3850,"record_offset":14524,"parent_record_offset":14500,"ancestor_record_offset":14388,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3850,"record_offset":14476,"parent_record_offset":14444,"ancestor_record_offset":14388,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/03850.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3850,"record_offset":14552,"parent_record_offset":14524,"ancestor_record_offset":14388,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/03850.jsonl.gz"}
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
// 完整结构及原始类型记录: gothic_balcony_door.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: gothic_balcony_door.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: gothic_balcony_door.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::gothic_balcony_door::generate_gothic_balcony_doors
// 信息 ID: be9651ec97db9512107d1cc0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 58df2e46a12fee5bb18b6ad9: void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>)> > @ 0x17cc499 -> 0x1c49290 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>)> @ 0x1c81854 -> 0x1c49290 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::gothic_balcony_door::get_door_holes
// 信息 ID: 76b0c1160bfd019881f8ec13
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::gothic_balcony_door::generate_gothic_balcony_doors; module=3850, inline_record=14388; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
