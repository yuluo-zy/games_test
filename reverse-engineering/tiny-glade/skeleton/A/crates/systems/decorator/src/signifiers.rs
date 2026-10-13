// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/decorator/src/signifiers.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: f0f054772ffe89cdc33382adb6dfab9cf0da353b8dcbbf393f5a1000cb250616
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: signifiers.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 3833
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_decorator::signifiers::cant_place_decorator_here::closure$0
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3833,"record_offset":2356,"rva":29585936,"va":5398295056}
// 编译实例 RVAs: 0x1c37210
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25545454","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398295056,"raw_hex":"e812357afe","size":5,"soft_fail":false,"target_va":5372749696,"va":5398295145,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25546515","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398295056,"raw_hex":"e8ed307afe","size":5,"soft_fail":false,"target_va":5372749712,"va":5398296222,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t4682019","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398295056,"raw_hex":"e823714700","size":5,"soft_fail":false,"target_va":5402978144,"va":5398296120,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t4623564","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398295056,"raw_hex":"e8cc8c4600","size":5,"soft_fail":false,"target_va":5402919072,"va":5398295503,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::world::command_queue::CommandQueue::get_raw
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-21049107","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398295056,"raw_hex":"e8edd0befe","size":5,"soft_fail":false,"target_va":5377246528,"va":5398295630,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::animation_duration
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-22938570","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398295056,"raw_hex":"e836fca1fe","size":5,"soft_fail":false,"target_va":5375356704,"va":5398295269,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-22938556","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398295056,"raw_hex":"e844fca1fe","size":5,"soft_fail":false,"target_va":5375356896,"va":5398295447,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::custom_shader
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-22938980","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398295056,"raw_hex":"e89cfaa1fe","size":5,"soft_fail":false,"target_va":5375356320,"va":5398295295,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-22939274","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398295056,"raw_hex":"e876f9a1fe","size":5,"soft_fail":false,"target_va":5375355984,"va":5398295253,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::position
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-22938832","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398295056,"raw_hex":"e830fba1fe","size":5,"soft_fail":false,"target_va":5375356560,"va":5398295387,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::rotation
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-22938810","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398295056,"raw_hex":"e846fba1fe","size":5,"soft_fail":false,"target_va":5375356608,"va":5398295413,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::scale
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-22939338","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398295056,"raw_hex":"e836f9a1fe","size":5,"soft_fail":false,"target_va":5375356656,"va":5398295989,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3833,"record_offset":2536,"parent_record_offset":2356,"ancestor_record_offset":2356,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/03833.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3833,"record_offset":5840,"parent_record_offset":2356,"ancestor_record_offset":2356,"inlinee":4441,"source_line":461,"archive":"_pdb/A/modules/03833.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::EntityCommands::insert
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3833,"record_offset":3508,"parent_record_offset":2356,"ancestor_record_offset":2356,"inlinee":14636,"source_line":945,"archive":"_pdb/A/modules/03833.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":3833,"record_offset":2684,"parent_record_offset":2356,"ancestor_record_offset":2356,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/03833.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3833,"record_offset":6232,"parent_record_offset":2356,"ancestor_record_offset":2356,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03833.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3833,"record_offset":3420,"parent_record_offset":2356,"ancestor_record_offset":2356,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/03833.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$4::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3833,"record_offset":3328,"parent_record_offset":2356,"ancestor_record_offset":2356,"inlinee":13205,"source_line":994,"archive":"_pdb/A/modules/03833.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3833,"record_offset":3240,"parent_record_offset":2356,"ancestor_record_offset":2356,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/03833.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3833,"record_offset":2592,"parent_record_offset":2568,"ancestor_record_offset":2356,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/03833.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3833,"record_offset":2616,"parent_record_offset":2592,"ancestor_record_offset":2356,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/03833.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3833,"record_offset":6304,"parent_record_offset":6280,"ancestor_record_offset":2356,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/03833.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3833,"record_offset":2568,"parent_record_offset":2536,"ancestor_record_offset":2356,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/03833.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3833,"record_offset":6280,"parent_record_offset":6256,"ancestor_record_offset":2356,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/03833.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":3833,"record_offset":6256,"parent_record_offset":6232,"ancestor_record_offset":2356,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/03833.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3833,"record_offset":3760,"parent_record_offset":3732,"ancestor_record_offset":2356,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/03833.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3833,"record_offset":3732,"parent_record_offset":3700,"ancestor_record_offset":2356,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/03833.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3833,"record_offset":3864,"parent_record_offset":3840,"ancestor_record_offset":2356,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/03833.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3833,"record_offset":3700,"parent_record_offset":3672,"ancestor_record_offset":2356,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/03833.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3833,"record_offset":2972,"parent_record_offset":2944,"ancestor_record_offset":2356,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/03833.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3833,"record_offset":2944,"parent_record_offset":2916,"ancestor_record_offset":2356,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/03833.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3833,"record_offset":3116,"parent_record_offset":2944,"ancestor_record_offset":2356,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/03833.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3833,"record_offset":2880,"parent_record_offset":2852,"ancestor_record_offset":2356,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/03833.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3833,"record_offset":5864,"parent_record_offset":5840,"ancestor_record_offset":2356,"inlinee":4442,"source_line":100,"archive":"_pdb/A/modules/03833.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3833,"record_offset":2852,"parent_record_offset":2824,"ancestor_record_offset":2356,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/03833.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3833,"record_offset":2796,"parent_record_offset":2768,"ancestor_record_offset":2356,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/03833.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3833,"record_offset":2824,"parent_record_offset":2796,"ancestor_record_offset":2356,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/03833.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3833,"record_offset":2768,"parent_record_offset":2740,"ancestor_record_offset":2356,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/03833.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":3833,"record_offset":2740,"parent_record_offset":2712,"ancestor_record_offset":2356,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/03833.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":3833,"record_offset":2712,"parent_record_offset":2684,"ancestor_record_offset":2356,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/03833.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3833,"record_offset":3840,"parent_record_offset":3632,"ancestor_record_offset":2356,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/03833.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3833,"record_offset":5888,"parent_record_offset":5864,"ancestor_record_offset":2356,"inlinee":4443,"source_line":819,"archive":"_pdb/A/modules/03833.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3833,"record_offset":3812,"parent_record_offset":3632,"ancestor_record_offset":2356,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/03833.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3833,"record_offset":3672,"parent_record_offset":3632,"ancestor_record_offset":2356,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/03833.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3833,"record_offset":3980,"parent_record_offset":3632,"ancestor_record_offset":2356,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/03833.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3833,"record_offset":2916,"parent_record_offset":2880,"ancestor_record_offset":2356,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/03833.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::add
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3833,"record_offset":3564,"parent_record_offset":3536,"ancestor_record_offset":2356,"inlinee":14638,"source_line":748,"archive":"_pdb/A/modules/03833.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::push
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3833,"record_offset":3596,"parent_record_offset":3564,"ancestor_record_offset":2356,"inlinee":12140,"source_line":501,"archive":"_pdb/A/modules/03833.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::add
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3833,"record_offset":3536,"parent_record_offset":3508,"ancestor_record_offset":2356,"inlinee":14637,"source_line":1100,"archive":"_pdb/A/modules/03833.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::CommandQueue::push
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":3833,"record_offset":4012,"parent_record_offset":3596,"ancestor_record_offset":2356,"inlinee":12149,"source_line":78,"archive":"_pdb/A/modules/03833.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::RawCommandQueue::push
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":3833,"record_offset":3632,"parent_record_offset":3596,"ancestor_record_offset":2356,"inlinee":12142,"source_line":157,"archive":"_pdb/A/modules/03833.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":3833,"record_offset":3176,"parent_record_offset":3152,"ancestor_record_offset":2356,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/03833.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3833,"record_offset":3268,"parent_record_offset":3240,"ancestor_record_offset":2356,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/03833.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3833,"record_offset":3448,"parent_record_offset":3420,"ancestor_record_offset":2356,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/03833.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$216::div
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3833,"record_offset":3360,"parent_record_offset":3328,"ancestor_record_offset":2356,"inlinee":5488,"source_line":502,"archive":"_pdb/A/modules/03833.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3833,"record_offset":3152,"parent_record_offset":2880,"ancestor_record_offset":2356,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/03833.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_unaligned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3833,"record_offset":3896,"parent_record_offset":3632,"ancestor_record_offset":2356,"inlinee":12143,"source_line":1546,"archive":"_pdb/A/modules/03833.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3833,"record_offset":2640,"parent_record_offset":2616,"ancestor_record_offset":2356,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/03833.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write_unaligned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3833,"record_offset":3920,"parent_record_offset":3896,"ancestor_record_offset":2356,"inlinee":12144,"source_line":1579,"archive":"_pdb/A/modules/03833.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::signifiers::cant_place_decorator_here
// 记录形式: compiled_procedure; instances=1
// 源码行: 13
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3833,"record_offset":848,"rva":29585392,"va":5398294512}
// 编译实例 RVAs: 0x1c36ff0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25544973","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398294512,"raw_hex":"e8f3367afe","size":5,"soft_fail":false,"target_va":5372749696,"va":5398294664,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25545229","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398294512,"raw_hex":"e8f3357afe","size":5,"soft_fail":false,"target_va":5372749712,"va":5398294936,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4683170","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398294512,"raw_hex":"e8a2754700","size":5,"soft_fail":false,"target_va":5402978144,"va":5398294969,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-20764738","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398294512,"raw_hex":"e8be27c3fe","size":5,"soft_fail":false,"target_va":5377529888,"va":5398294621,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierFactory::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-22939137","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398294512,"raw_hex":"e8fff9a1fe","size":5,"soft_fail":false,"target_va":5375355744,"va":5398294876,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::send
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-16021285","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398294512,"raw_hex":"e8db880bff","size":5,"soft_fail":false,"target_va":5382273632,"va":5398294912,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":3833,"record_offset":1000,"parent_record_offset":848,"ancestor_record_offset":848,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/03833.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":3833,"record_offset":1080,"parent_record_offset":848,"ancestor_record_offset":848,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/03833.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3833,"record_offset":1952,"parent_record_offset":848,"ancestor_record_offset":848,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03833.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":3833,"record_offset":1304,"parent_record_offset":848,"ancestor_record_offset":848,"inlinee":14676,"source_line":659,"archive":"_pdb/A/modules/03833.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3833,"record_offset":1200,"parent_record_offset":1176,"ancestor_record_offset":848,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/03833.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3833,"record_offset":1224,"parent_record_offset":1200,"ancestor_record_offset":848,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/03833.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3833,"record_offset":2104,"parent_record_offset":2080,"ancestor_record_offset":848,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/03833.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3833,"record_offset":1144,"parent_record_offset":1112,"ancestor_record_offset":848,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/03833.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3833,"record_offset":1176,"parent_record_offset":1144,"ancestor_record_offset":848,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/03833.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3833,"record_offset":2080,"parent_record_offset":2024,"ancestor_record_offset":848,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/03833.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":3833,"record_offset":1112,"parent_record_offset":1080,"ancestor_record_offset":848,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/03833.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":3833,"record_offset":1048,"parent_record_offset":1024,"ancestor_record_offset":848,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/03833.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3833,"record_offset":2052,"parent_record_offset":2024,"ancestor_record_offset":848,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/03833.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3833,"record_offset":2024,"parent_record_offset":2000,"ancestor_record_offset":848,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/03833.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3833,"record_offset":1888,"parent_record_offset":1328,"ancestor_record_offset":848,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/03833.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3833,"record_offset":1796,"parent_record_offset":1740,"ancestor_record_offset":848,"inlinee":21747,"source_line":20,"archive":"_pdb/A/modules/03833.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3833,"record_offset":1372,"parent_record_offset":1328,"ancestor_record_offset":848,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/03833.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3833,"record_offset":1456,"parent_record_offset":1400,"ancestor_record_offset":848,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/03833.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3833,"record_offset":1660,"parent_record_offset":1328,"ancestor_record_offset":848,"inlinee":14685,"source_line":54,"archive":"_pdb/A/modules/03833.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3833,"record_offset":1024,"parent_record_offset":1000,"ancestor_record_offset":848,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/03833.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3833,"record_offset":1976,"parent_record_offset":1952,"ancestor_record_offset":848,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03833.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3833,"record_offset":1252,"parent_record_offset":1224,"ancestor_record_offset":848,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/03833.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":3833,"record_offset":1328,"parent_record_offset":1304,"ancestor_record_offset":848,"inlinee":14681,"source_line":731,"archive":"_pdb/A/modules/03833.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":3833,"record_offset":1400,"parent_record_offset":1328,"ancestor_record_offset":848,"inlinee":14682,"source_line":220,"archive":"_pdb/A/modules/03833.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":3833,"record_offset":1544,"parent_record_offset":1516,"ancestor_record_offset":848,"inlinee":21746,"source_line":42,"archive":"_pdb/A/modules/03833.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":3833,"record_offset":1516,"parent_record_offset":1328,"ancestor_record_offset":848,"inlinee":21745,"source_line":66,"archive":"_pdb/A/modules/03833.jsonl.gz"}
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
// 完整结构及原始类型记录: signifiers.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: signifiers.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: signifiers.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: system_decorator::signifiers::cant_place_decorator_here
// 信息 ID: 62a5e2e52b5f9e55113ce635
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::blueprint::validate_blueprints @ 0x17a2c23 -> 0x1c36ff0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_place_decorator::ui_place_decorator @ 0x1c6e2fe -> 0x1c36ff0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::signifiers::cant_place_decorator_here::closure$0
// 信息 ID: 9e55ac314369957fec780b49
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::signifiers::cant_place_decorator_here", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::signifiers::cant_place_decorator_here::closure_env$0 <- core::ops::function::FnOnce::call_once<system_decorator::signifiers::cant_place_decorator_here::closure_env$0,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::signifiers::cant_place_decorator_here::closure_env$0 <- core::ptr::drop_in_place<system_decorator::signifiers::cant_place_decorator_here::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 2 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<system_decorator::signifiers::cant_place_decorator_here::closure_env$0,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0x1c36fe2 -> 0x1c37210 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
