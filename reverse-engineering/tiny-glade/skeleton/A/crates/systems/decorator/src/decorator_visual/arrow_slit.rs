// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/decorator/src/decorator_visual/arrow_slit.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 5218fb5315cfdc08fba5193bebe2840ab73daf455d30e6c512daf0415b056748
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: arrow_slit.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 2749, 3856
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
// 函数符号: system_decorator::decorator_visual::arrow_slit::generate_arrow_slits
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3856,"record_offset":3608,"rva":29689968,"va":5398399088}
// 编译实例 RVAs: 0x1c50870
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dde70
//     owner=unresolved_namespace; count=1
//     target_aliases=30 candidates; complete list: arrow_slit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28782621","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398399088,"raw_hex":"e8e3cf48fe","size":5,"soft_fail":false,"target_va":5369618032,"va":5398400648,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14010da30
//     owner=unresolved_namespace; count=2
//     target_aliases=12 candidates; complete list: arrow_slit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28588044","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398399088,"raw_hex":"e8f4c74bfe","size":5,"soft_fail":false,"target_va":5369813552,"va":5398401591,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402ce6a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: arrow_slit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26754794","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398399088,"raw_hex":"e816c167fe","size":5,"soft_fail":false,"target_va":5371651744,"va":5398406533,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043c290
//     owner=unresolved_namespace; count=1
//     target_aliases=53 candidates; complete list: arrow_slit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25253951","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398399088,"raw_hex":"e8c1a77efe","size":5,"soft_fail":false,"target_va":5373149840,"va":5398403786,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140593250
//     owner=unresolved_namespace; count=3
//     target_aliases=15 candidates; complete list: arrow_slit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23846399","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398399088,"raw_hex":"e8012294fe","size":5,"soft_fail":false,"target_va":5374554704,"va":5398401098,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b9c30
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: arrow_slit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23687163","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398399088,"raw_hex":"e8059096fe","size":5,"soft_fail":false,"target_va":5374712880,"va":5398400038,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405bab80
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: arrow_slit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23684741","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398399088,"raw_hex":"e87b9996fe","size":5,"soft_fail":false,"target_va":5374716800,"va":5398401536,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140633130
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: arrow_slit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23191348","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398399088,"raw_hex":"e8cc209efe","size":5,"soft_fail":false,"target_va":5375209776,"va":5398401119,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140633eb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: arrow_slit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23186782","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398399088,"raw_hex":"e8a2329efe","size":5,"soft_fail":false,"target_va":5375213232,"va":5398400009,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da760
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: arrow_slit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21457496","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398399088,"raw_hex":"e8a895b8fe","size":5,"soft_fail":false,"target_va":5376943968,"va":5398401459,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da780
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: arrow_slit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21461201","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398399088,"raw_hex":"e82f87b8fe","size":5,"soft_fail":false,"target_va":5376944000,"va":5398405196,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c96d80
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: arrow_slit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16492584","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398399088,"raw_hex":"e8d85704ff","size":5,"soft_fail":false,"target_va":5381909888,"va":5398402467,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d06720
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: arrow_slit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16034143","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398399088,"raw_hex":"e8a1560bff","size":5,"soft_fail":false,"target_va":5382367008,"va":5398401146,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t-5023425","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398399088,"raw_hex":"e83f59b3ff","size":5,"soft_fail":false,"target_va":5393378400,"va":5398401820,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4572524","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398399088,"raw_hex":"e86cc54500","size":5,"soft_fail":false,"target_va":5402978688,"va":5398406159,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4573052","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398399088,"raw_hex":"e87cc74500","size":5,"soft_fail":false,"target_va":5402978912,"va":5398405855,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t4573080","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398399088,"raw_hex":"e898c74500","size":5,"soft_fail":false,"target_va":5402979152,"va":5398406067,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t4573233","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398399088,"raw_hex":"e831c84500","size":5,"soft_fail":false,"target_va":5402978944,"va":5398405706,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-2177","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398399088,"raw_hex":"e87ff7ffff","size":5,"soft_fail":false,"target_va":5398397728,"va":5398399900,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4580450","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398399088,"raw_hex":"e862e44500","size":5,"soft_fail":false,"target_va":5402980128,"va":5398399673,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1318","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398399088,"raw_hex":"e8dafaffff","size":5,"soft_fail":false,"target_va":5398397920,"va":5398399233,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t4581341","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398399088,"raw_hex":"e8dde74500","size":5,"soft_fail":false,"target_va":5402980656,"va":5398399310,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorInfo::dst_as_wall
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-23830196","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398399088,"raw_hex":"e84c6194fe","size":5,"soft_fail":false,"target_va":5374570992,"va":5398401183,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::subtype
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-23187796","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398399088,"raw_hex":"e8ac2e9efe","size":5,"soft_fail":false,"target_va":5375212688,"va":5398400479,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::dormers::DormerHoleSsbo::gothic
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-23584516","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398399088,"raw_hex":"e8fc2098fe","size":5,"soft_fail":false,"target_va":5374821040,"va":5398405551,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-22042933","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398399088,"raw_hex":"e8cba6affe","size":5,"soft_fail":false,"target_va":5376356912,"va":5398399840,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-22039046","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398399088,"raw_hex":"e8fab5affe","size":5,"soft_fail":false,"target_va":5376360560,"va":5398399601,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-22043074","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398399088,"raw_hex":"e83ea6affe","size":5,"soft_fail":false,"target_va":5376356144,"va":5398399213,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::fast_local::Key$LT$T$GT$::try_initialize::hcc99216871ec738f (.llvm.16000913382889057913)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-16109818","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398399088,"raw_hex":"e8062f0aff","size":5,"soft_fail":false,"target_va":5382289808,"va":5398399621,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::blueprint::DecoratorBlueprint::set_colliders
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-4912468","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398399088,"raw_hex":"e8ac0ab5ff","size":5,"soft_fail":false,"target_va":5393490672,"va":5398403135,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::blueprint::DecoratorBlueprints::insert
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-4910083","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398399088,"raw_hex":"e8fd13b5ff","size":5,"soft_fail":false,"target_va":5393491488,"va":5398401566,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::decorator_visual::gothic_wall_window::get_gothic_hat_bones
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-67113","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398399088,"raw_hex":"e8d7f9feff","size":5,"soft_fail":false,"target_va":5398334144,"va":5398401252,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle3d::extent
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-20452273","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398399088,"raw_hex":"e84fecc7fe","size":5,"soft_fail":false,"target_va":5377952848,"va":5398405116,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::xz_projective_rotation
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21458107","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398399088,"raw_hex":"e84593b8fe","size":5,"soft_fail":false,"target_va":5376943376,"va":5398401478,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3856,"record_offset":7716,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3856,"record_offset":11540,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":3856,"record_offset":11596,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":3856,"record_offset":6612,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":3856,"record_offset":6584,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3856,"record_offset":17148,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::bbx_ls
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3856,"record_offset":10968,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":21725,"source_line":267,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::collision_mesh
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3856,"record_offset":11844,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":60285,"source_line":274,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::interaction_mesh
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3856,"record_offset":13308,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":60287,"source_line":285,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3856,"record_offset":6528,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":21526,"source_line":249,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":3856,"record_offset":15256,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":21375,"source_line":96,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":3856,"record_offset":15284,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":14676,"source_line":659,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3856,"record_offset":11324,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":21371,"source_line":450,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3856,"record_offset":11512,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":14690,"source_line":814,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3856,"record_offset":11132,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":21436,"source_line":742,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3856,"record_offset":17120,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":14693,"source_line":76,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3856,"record_offset":16936,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$13::add_assign
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3856,"record_offset":15932,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":21924,"source_line":1092,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3856,"record_offset":16696,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3856,"record_offset":5480,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3856,"record_offset":3880,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3856,"record_offset":4468,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$36::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3856,"record_offset":7356,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":8927,"source_line":2181,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$39::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3856,"record_offset":7856,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":8642,"source_line":2233,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3856,"record_offset":8488,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":17187,"source_line":1342,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::blueprint::DecoratorBlueprint::set_outline
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/blueprint.rs
//     evidence={"module":3856,"record_offset":16020,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":60163,"source_line":214,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::mark_as_preview
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":3856,"record_offset":11680,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":60170,"source_line":72,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::mark_as_window_glass
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":3856,"record_offset":16224,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":60172,"source_line":153,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::new
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":3856,"record_offset":11652,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":60169,"source_line":49,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::center
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":3856,"record_offset":16668,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":22795,"source_line":60,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::width
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":3856,"record_offset":11624,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":21735,"source_line":70,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3856,"record_offset":7792,"parent_record_offset":7760,"ancestor_record_offset":3608,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":3856,"record_offset":13128,"parent_record_offset":11884,"ancestor_record_offset":3608,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":3856,"record_offset":17172,"parent_record_offset":17148,"ancestor_record_offset":3608,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3856,"record_offset":7760,"parent_record_offset":7716,"ancestor_record_offset":3608,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3856,"record_offset":11564,"parent_record_offset":11540,"ancestor_record_offset":3608,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3856,"record_offset":5880,"parent_record_offset":5852,"ancestor_record_offset":3608,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3856,"record_offset":5932,"parent_record_offset":5908,"ancestor_record_offset":3608,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3856,"record_offset":5908,"parent_record_offset":5880,"ancestor_record_offset":3608,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3856,"record_offset":5820,"parent_record_offset":5792,"ancestor_record_offset":3608,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3856,"record_offset":5852,"parent_record_offset":5820,"ancestor_record_offset":3608,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3856,"record_offset":6024,"parent_record_offset":5852,"ancestor_record_offset":3608,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3856,"record_offset":6136,"parent_record_offset":6112,"ancestor_record_offset":3608,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3856,"record_offset":10748,"parent_record_offset":10724,"ancestor_record_offset":3608,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":3856,"record_offset":6552,"parent_record_offset":6528,"ancestor_record_offset":3608,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3856,"record_offset":6976,"parent_record_offset":6944,"ancestor_record_offset":3608,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3856,"record_offset":10208,"parent_record_offset":10184,"ancestor_record_offset":3608,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3856,"record_offset":7040,"parent_record_offset":7004,"ancestor_record_offset":3608,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3856,"record_offset":15724,"parent_record_offset":15308,"ancestor_record_offset":3608,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3856,"record_offset":15632,"parent_record_offset":15576,"ancestor_record_offset":3608,"inlinee":21747,"source_line":20,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3856,"record_offset":15404,"parent_record_offset":15380,"ancestor_record_offset":3608,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3856,"record_offset":15436,"parent_record_offset":15352,"ancestor_record_offset":3608,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3856,"record_offset":15496,"parent_record_offset":15308,"ancestor_record_offset":3608,"inlinee":14685,"source_line":54,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":3856,"record_offset":16108,"parent_record_offset":16020,"ancestor_record_offset":3608,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":3856,"record_offset":11088,"parent_record_offset":11064,"ancestor_record_offset":3608,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::BuildHasher::hash_one
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":3856,"record_offset":8636,"parent_record_offset":8612,"ancestor_record_offset":3608,"inlinee":8489,"source_line":696,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":3856,"record_offset":8864,"parent_record_offset":8840,"ancestor_record_offset":3608,"inlinee":8461,"source_line":395,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$13::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":3856,"record_offset":8840,"parent_record_offset":8816,"ancestor_record_offset":3608,"inlinee":6472,"source_line":806,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$5::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":3856,"record_offset":8792,"parent_record_offset":8636,"ancestor_record_offset":3608,"inlinee":5832,"source_line":941,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::Hasher::new_with_keys
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3856,"record_offset":8720,"parent_record_offset":8692,"ancestor_record_offset":3608,"inlinee":8426,"source_line":190,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::Hasher::reset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3856,"record_offset":8748,"parent_record_offset":8720,"ancestor_record_offset":3608,"inlinee":8427,"source_line":205,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::SipHasher13::new_with_keys
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3856,"record_offset":8692,"parent_record_offset":8668,"ancestor_record_offset":3608,"inlinee":8424,"source_line":183,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3856,"record_offset":9016,"parent_record_offset":8992,"ancestor_record_offset":3608,"inlinee":8438,"source_line":246,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3856,"record_offset":8944,"parent_record_offset":8920,"ancestor_record_offset":3608,"inlinee":8436,"source_line":236,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3856,"record_offset":9040,"parent_record_offset":9016,"ancestor_record_offset":3608,"inlinee":8440,"source_line":315,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3856,"record_offset":9088,"parent_record_offset":9040,"ancestor_record_offset":3608,"inlinee":8442,"source_line":365,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3856,"record_offset":9480,"parent_record_offset":9040,"ancestor_record_offset":3608,"inlinee":8443,"source_line":370,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":3856,"record_offset":10232,"parent_record_offset":10208,"ancestor_record_offset":3608,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":3856,"record_offset":5956,"parent_record_offset":5932,"ancestor_record_offset":3608,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3856,"record_offset":8888,"parent_record_offset":8864,"ancestor_record_offset":3608,"inlinee":8462,"source_line":2859,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":3856,"record_offset":4104,"parent_record_offset":4008,"ancestor_record_offset":3608,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3856,"record_offset":9188,"parent_record_offset":9088,"ancestor_record_offset":3608,"inlinee":7983,"source_line":234,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3856,"record_offset":9156,"parent_record_offset":9088,"ancestor_record_offset":3608,"inlinee":6614,"source_line":1639,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":3856,"record_offset":6872,"parent_record_offset":6848,"ancestor_record_offset":3608,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3856,"record_offset":11260,"parent_record_offset":11164,"ancestor_record_offset":3608,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3856,"record_offset":16968,"parent_record_offset":16936,"ancestor_record_offset":3608,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$308::add_assign
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3856,"record_offset":15960,"parent_record_offset":15932,"ancestor_record_offset":3608,"inlinee":14664,"source_line":761,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3856,"record_offset":11204,"parent_record_offset":11164,"ancestor_record_offset":3608,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":3856,"record_offset":7320,"parent_record_offset":7288,"ancestor_record_offset":3608,"inlinee":4428,"source_line":304,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3856,"record_offset":5716,"parent_record_offset":5688,"ancestor_record_offset":3608,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3856,"record_offset":16052,"parent_record_offset":16020,"ancestor_record_offset":3608,"inlinee":4710,"source_line":651,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3856,"record_offset":16076,"parent_record_offset":16052,"ancestor_record_offset":3608,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3856,"record_offset":5064,"parent_record_offset":4604,"ancestor_record_offset":3608,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3856,"record_offset":5764,"parent_record_offset":5588,"ancestor_record_offset":3608,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3856,"record_offset":11032,"parent_record_offset":11004,"ancestor_record_offset":3608,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3856,"record_offset":4416,"parent_record_offset":4388,"ancestor_record_offset":3608,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3856,"record_offset":6088,"parent_record_offset":5792,"ancestor_record_offset":3608,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3856,"record_offset":7124,"parent_record_offset":7100,"ancestor_record_offset":3608,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3856,"record_offset":7100,"parent_record_offset":7076,"ancestor_record_offset":3608,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3856,"record_offset":4840,"parent_record_offset":4816,"ancestor_record_offset":3608,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3856,"record_offset":4816,"parent_record_offset":4792,"ancestor_record_offset":3608,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3856,"record_offset":5024,"parent_record_offset":4992,"ancestor_record_offset":3608,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3856,"record_offset":5980,"parent_record_offset":5956,"ancestor_record_offset":3608,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3856,"record_offset":6488,"parent_record_offset":5552,"ancestor_record_offset":3608,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3856,"record_offset":4040,"parent_record_offset":4008,"ancestor_record_offset":3608,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3856,"record_offset":4068,"parent_record_offset":4040,"ancestor_record_offset":3608,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3856,"record_offset":4792,"parent_record_offset":4768,"ancestor_record_offset":3608,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3856,"record_offset":5120,"parent_record_offset":5096,"ancestor_record_offset":3608,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3856,"record_offset":4768,"parent_record_offset":4660,"ancestor_record_offset":3608,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":3856,"record_offset":4008,"parent_record_offset":3980,"ancestor_record_offset":3608,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":3856,"record_offset":4500,"parent_record_offset":4468,"ancestor_record_offset":3608,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":3856,"record_offset":4632,"parent_record_offset":4604,"ancestor_record_offset":3608,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":3856,"record_offset":4604,"parent_record_offset":4572,"ancestor_record_offset":3608,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3856,"record_offset":4548,"parent_record_offset":4524,"ancestor_record_offset":3608,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3856,"record_offset":4572,"parent_record_offset":4548,"ancestor_record_offset":3608,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3856,"record_offset":4524,"parent_record_offset":4500,"ancestor_record_offset":3608,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3856,"record_offset":5096,"parent_record_offset":4572,"ancestor_record_offset":3608,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3856,"record_offset":4204,"parent_record_offset":4172,"ancestor_record_offset":3608,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3856,"record_offset":4364,"parent_record_offset":4204,"ancestor_record_offset":3608,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3856,"record_offset":4172,"parent_record_offset":4148,"ancestor_record_offset":3608,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3856,"record_offset":3916,"parent_record_offset":3880,"ancestor_record_offset":3608,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3856,"record_offset":4388,"parent_record_offset":4364,"ancestor_record_offset":3608,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3856,"record_offset":3980,"parent_record_offset":3944,"ancestor_record_offset":3608,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3856,"record_offset":3944,"parent_record_offset":3916,"ancestor_record_offset":3608,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3856,"record_offset":4660,"parent_record_offset":4632,"ancestor_record_offset":3608,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3856,"record_offset":4908,"parent_record_offset":4660,"ancestor_record_offset":3608,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3856,"record_offset":4880,"parent_record_offset":4660,"ancestor_record_offset":3608,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3856,"record_offset":4712,"parent_record_offset":4660,"ancestor_record_offset":3608,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3856,"record_offset":13156,"parent_record_offset":13128,"ancestor_record_offset":3608,"inlinee":5317,"source_line":2665,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3856,"record_offset":17208,"parent_record_offset":17172,"ancestor_record_offset":3608,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3856,"record_offset":13180,"parent_record_offset":13156,"ancestor_record_offset":3608,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3856,"record_offset":17232,"parent_record_offset":17208,"ancestor_record_offset":3608,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3856,"record_offset":17264,"parent_record_offset":17172,"ancestor_record_offset":3608,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::collision_mesh
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3856,"record_offset":11884,"parent_record_offset":11844,"ancestor_record_offset":3608,"inlinee":60286,"source_line":225,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::get_collision_bbx_ls
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3856,"record_offset":11004,"parent_record_offset":10968,"ancestor_record_offset":3608,"inlinee":21727,"source_line":199,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::interaction_mesh
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3856,"record_offset":13348,"parent_record_offset":13308,"ancestor_record_offset":3608,"inlinee":60288,"source_line":234,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$21::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3856,"record_offset":12820,"parent_record_offset":12792,"ancestor_record_offset":3608,"inlinee":26328,"source_line":78,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$3::collision_mesh::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3856,"record_offset":13240,"parent_record_offset":13216,"ancestor_record_offset":3608,"inlinee":60292,"source_line":228,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$3::get_collision_bbx_ls::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3856,"record_offset":11064,"parent_record_offset":11032,"ancestor_record_offset":3608,"inlinee":21755,"source_line":202,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$3::interaction_mesh::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3856,"record_offset":14704,"parent_record_offset":14680,"ancestor_record_offset":3608,"inlinee":60290,"source_line":237,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$6::hash
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":3856,"record_offset":8816,"parent_record_offset":8792,"ancestor_record_offset":3608,"inlinee":25074,"source_line":65,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":3856,"record_offset":10772,"parent_record_offset":10748,"ancestor_record_offset":3608,"inlinee":8043,"source_line":65,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":3856,"record_offset":15308,"parent_record_offset":15284,"ancestor_record_offset":3608,"inlinee":14681,"source_line":731,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":3856,"record_offset":15752,"parent_record_offset":15308,"ancestor_record_offset":3608,"inlinee":14682,"source_line":220,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3856,"record_offset":11408,"parent_record_offset":11376,"ancestor_record_offset":3608,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3856,"record_offset":11376,"parent_record_offset":11348,"ancestor_record_offset":3608,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3856,"record_offset":11348,"parent_record_offset":11324,"ancestor_record_offset":3608,"inlinee":21372,"source_line":399,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3856,"record_offset":11164,"parent_record_offset":11132,"ancestor_record_offset":3608,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":3856,"record_offset":15380,"parent_record_offset":15352,"ancestor_record_offset":3608,"inlinee":21746,"source_line":42,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":3856,"record_offset":15352,"parent_record_offset":15308,"ancestor_record_offset":3608,"inlinee":21745,"source_line":66,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":3856,"record_offset":10724,"parent_record_offset":10700,"ancestor_record_offset":3608,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3856,"record_offset":8556,"parent_record_offset":8520,"ancestor_record_offset":3608,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3856,"record_offset":8580,"parent_record_offset":8556,"ancestor_record_offset":3608,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3856,"record_offset":7404,"parent_record_offset":7380,"ancestor_record_offset":3608,"inlinee":8929,"source_line":756,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3856,"record_offset":10700,"parent_record_offset":10476,"ancestor_record_offset":3608,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$60::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3856,"record_offset":6664,"parent_record_offset":6636,"ancestor_record_offset":3608,"inlinee":8644,"source_line":4727,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3856,"record_offset":8612,"parent_record_offset":8580,"ancestor_record_offset":3608,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3856,"record_offset":10612,"parent_record_offset":10588,"ancestor_record_offset":3608,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3856,"record_offset":10588,"parent_record_offset":10476,"ancestor_record_offset":3608,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3856,"record_offset":10528,"parent_record_offset":10504,"ancestor_record_offset":3608,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3856,"record_offset":7076,"parent_record_offset":6724,"ancestor_record_offset":3608,"inlinee":8358,"source_line":561,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3856,"record_offset":10908,"parent_record_offset":10104,"ancestor_record_offset":3608,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3856,"record_offset":7512,"parent_record_offset":7452,"ancestor_record_offset":3608,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3856,"record_offset":6724,"parent_record_offset":6692,"ancestor_record_offset":3608,"inlinee":8357,"source_line":3827,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3856,"record_offset":10504,"parent_record_offset":10476,"ancestor_record_offset":3608,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3856,"record_offset":10080,"parent_record_offset":10056,"ancestor_record_offset":3608,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3856,"record_offset":10056,"parent_record_offset":8580,"ancestor_record_offset":3608,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3856,"record_offset":7428,"parent_record_offset":7404,"ancestor_record_offset":3608,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3856,"record_offset":7484,"parent_record_offset":7452,"ancestor_record_offset":3608,"inlinee":6422,"source_line":2597,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3856,"record_offset":10104,"parent_record_offset":10080,"ancestor_record_offset":3608,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3856,"record_offset":7452,"parent_record_offset":7428,"ancestor_record_offset":3608,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3856,"record_offset":6812,"parent_record_offset":6776,"ancestor_record_offset":3608,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3856,"record_offset":6848,"parent_record_offset":6812,"ancestor_record_offset":3608,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3856,"record_offset":6908,"parent_record_offset":6776,"ancestor_record_offset":3608,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3856,"record_offset":6776,"parent_record_offset":6724,"ancestor_record_offset":3608,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3856,"record_offset":10156,"parent_record_offset":10104,"ancestor_record_offset":3608,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$28::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3856,"record_offset":6692,"parent_record_offset":6664,"ancestor_record_offset":3608,"inlinee":8355,"source_line":4127,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3856,"record_offset":10476,"parent_record_offset":10104,"ancestor_record_offset":3608,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3856,"record_offset":10184,"parent_record_offset":10104,"ancestor_record_offset":3608,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3856,"record_offset":6944,"parent_record_offset":6724,"ancestor_record_offset":3608,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3856,"record_offset":10268,"parent_record_offset":10104,"ancestor_record_offset":3608,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3856,"record_offset":10816,"parent_record_offset":10104,"ancestor_record_offset":3608,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3856,"record_offset":7004,"parent_record_offset":6944,"ancestor_record_offset":3608,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3856,"record_offset":5516,"parent_record_offset":5480,"ancestor_record_offset":3608,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":3856,"record_offset":5628,"parent_record_offset":5588,"ancestor_record_offset":3608,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3856,"record_offset":5792,"parent_record_offset":5588,"ancestor_record_offset":3608,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3856,"record_offset":6060,"parent_record_offset":5792,"ancestor_record_offset":3608,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3856,"record_offset":8520,"parent_record_offset":8488,"ancestor_record_offset":3608,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3856,"record_offset":7380,"parent_record_offset":7356,"ancestor_record_offset":3608,"inlinee":8928,"source_line":518,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$39::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3856,"record_offset":6636,"parent_record_offset":6612,"ancestor_record_offset":3608,"inlinee":8642,"source_line":2233,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":3856,"record_offset":11440,"parent_record_offset":11408,"ancestor_record_offset":3608,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$1::build_hasher
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":3856,"record_offset":8668,"parent_record_offset":8636,"ancestor_record_offset":3608,"inlinee":8422,"source_line":84,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":3856,"record_offset":8992,"parent_record_offset":8636,"ancestor_record_offset":3608,"inlinee":8437,"source_line":142,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":3856,"record_offset":8920,"parent_record_offset":8864,"ancestor_record_offset":3608,"inlinee":8434,"source_line":132,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":3856,"record_offset":5656,"parent_record_offset":5628,"ancestor_record_offset":3608,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":3856,"record_offset":5688,"parent_record_offset":5656,"ancestor_record_offset":3608,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":3856,"record_offset":5588,"parent_record_offset":5552,"ancestor_record_offset":3608,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":3856,"record_offset":5552,"parent_record_offset":5516,"ancestor_record_offset":3608,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/03856.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::arrow_slit::arrow_slit_profile_curve
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 238
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15EE3
// 首条位置: {"module":2749,"inlinee":89827}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2749,"record_offset":25128,"parent_record_offset":25100,"ancestor_record_offset":25100,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/02749.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":2749,"record_offset":25280,"parent_record_offset":25100,"ancestor_record_offset":25100,"inlinee":4441,"source_line":461,"archive":"_pdb/A/modules/02749.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2749,"record_offset":25184,"parent_record_offset":25160,"ancestor_record_offset":25100,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/02749.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2749,"record_offset":25208,"parent_record_offset":25184,"ancestor_record_offset":25100,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/02749.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2749,"record_offset":25160,"parent_record_offset":25128,"ancestor_record_offset":25100,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/02749.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":2749,"record_offset":25304,"parent_record_offset":25280,"ancestor_record_offset":25100,"inlinee":4442,"source_line":100,"archive":"_pdb/A/modules/02749.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2749,"record_offset":25328,"parent_record_offset":25304,"ancestor_record_offset":25100,"inlinee":4443,"source_line":819,"archive":"_pdb/A/modules/02749.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2749,"record_offset":25236,"parent_record_offset":25208,"ancestor_record_offset":25100,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/02749.jsonl.gz"}
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
// 完整结构及原始类型记录: arrow_slit.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: arrow_slit.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: arrow_slit.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::arrow_slit::arrow_slit_profile_curve
// 信息 ID: e314846ed38d7926ee7fb1be
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs; module=2749, inline_record=25100; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::arrow_slit::generate_arrow_slits
// 信息 ID: 0639b404d65fc72172f85095
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 88d0867f7d3735af30de04ed: void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>)> > @ 0x17ce29f -> 0x1c50870 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>)> @ 0x1c827e7 -> 0x1c50870 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
