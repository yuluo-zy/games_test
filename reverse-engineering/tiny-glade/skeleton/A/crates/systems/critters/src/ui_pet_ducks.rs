// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/critters/src/ui_pet_ducks.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 0eee71b7c650d8955a669c4df0985255f38c4edfad1ecdd8b4d2f7457f330b4b
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: ui_pet_ducks.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 3619
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
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_critters::ui_pet_ducks::ui_pet_ducks
// 记录形式: compiled_procedure; instances=1
// 源码行: 29
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3619,"record_offset":1728,"rva":28553248,"va":5397262368}
// 编译实例 RVAs: 0x1b3b020
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: ui_pet_ducks.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24514865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397262368,"raw_hex":"e8cfee89fe","size":5,"soft_fail":false,"target_va":5372749696,"va":5397264556,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: ui_pet_ducks.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24515164","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397262368,"raw_hex":"e8a4ed89fe","size":5,"soft_fail":false,"target_va":5372749712,"va":5397264871,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406edb80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_pet_ducks.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21292026","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397262368,"raw_hex":"e8061cbbfe","size":5,"soft_fail":false,"target_va":5375974272,"va":5397266293,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141df9930
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_pet_ducks.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2876308","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397262368,"raw_hex":"e894e32b00","size":5,"soft_fail":false,"target_va":5400140080,"va":5397263767,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t5712475","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397262368,"raw_hex":"e85b2a5700","size":5,"soft_fail":false,"target_va":5402978144,"va":5397265664,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-19733724","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397262368,"raw_hex":"e824e3d2fe","size":5,"soft_fail":false,"target_va":5377529888,"va":5397263607,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::audio_resources::AudioInstance::start_if_not_playing
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-22582865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397262368,"raw_hex":"e8af69a7fe","size":5,"soft_fail":false,"target_va":5374682656,"va":5397265516,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::camera_rig::MainCamera::eye_position
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-23210157","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397262368,"raw_hex":"e853d79dfe","size":5,"soft_fail":false,"target_va":5374053232,"va":5397263384,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::camera_rig::MainCamera::mode
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-23213940","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397262368,"raw_hex":"e88cc89dfe","size":5,"soft_fail":false,"target_va":5374048944,"va":5397262879,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::ui_state::UiWidgetContext::dragged
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21714532","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397262368,"raw_hex":"e89ca9b4fe","size":5,"soft_fail":false,"target_va":5375550352,"va":5397264879,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::ui_state::UiWidgetContext::intersecting
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21713420","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397262368,"raw_hex":"e8f4adb4fe","size":5,"soft_fail":false,"target_va":5375550384,"va":5397263799,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::ui_state::UiWidgetContext::is_active
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21713484","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397262368,"raw_hex":"e8b4adb4fe","size":5,"soft_fail":false,"target_va":5375550448,"va":5397263927,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::UiSounds::empty
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21492155","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397262368,"raw_hex":"e8450eb8fe","size":5,"soft_fail":false,"target_va":5375771552,"va":5397263702,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::WidgetParams::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21492524","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397262368,"raw_hex":"e8d40cb8fe","size":5,"soft_fail":false,"target_va":5375771168,"va":5397263687,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::WidgetParams::no_history_recording
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21492354","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397262368,"raw_hex":"e87e0db8fe","size":5,"soft_fail":false,"target_va":5375771392,"va":5397263741,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::WidgetParams::sounds
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21492448","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397262368,"raw_hex":"e8200db8fe","size":5,"soft_fail":false,"target_va":5375771280,"va":5397263723,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierFactory::new
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-21909019","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397262368,"raw_hex":"e8e5b1b1fe","size":5,"soft_fail":false,"target_va":5375355744,"va":5397264758,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::send
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-14991179","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397262368,"raw_hex":"e8b5401bff","size":5,"soft_fail":false,"target_va":5382273632,"va":5397264806,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":3619,"record_offset":3660,"parent_record_offset":1728,"ancestor_record_offset":1728,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":3619,"record_offset":4644,"parent_record_offset":1728,"ancestor_record_offset":1728,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3619,"record_offset":3604,"parent_record_offset":1728,"ancestor_record_offset":1728,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3619,"record_offset":2108,"parent_record_offset":1728,"ancestor_record_offset":1728,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3619,"record_offset":2196,"parent_record_offset":1728,"ancestor_record_offset":1728,"inlinee":14613,"source_line":881,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":3619,"record_offset":1908,"parent_record_offset":1728,"ancestor_record_offset":1728,"inlinee":14631,"source_line":473,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":3619,"record_offset":2164,"parent_record_offset":1728,"ancestor_record_offset":1728,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3619,"record_offset":5356,"parent_record_offset":1728,"ancestor_record_offset":1728,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::mat4::Mat4::transform_point3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat4.rs
//     evidence={"module":3619,"record_offset":6612,"parent_record_offset":1728,"ancestor_record_offset":1728,"inlinee":23594,"source_line":1098,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::from_mat4
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":3619,"record_offset":4008,"parent_record_offset":1728,"ancestor_record_offset":1728,"inlinee":108587,"source_line":267,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::impl$7::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":3619,"record_offset":4868,"parent_record_offset":1728,"ancestor_record_offset":1728,"inlinee":59541,"source_line":846,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3619,"record_offset":3408,"parent_record_offset":1728,"ancestor_record_offset":1728,"inlinee":21439,"source_line":462,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3619,"record_offset":3952,"parent_record_offset":1728,"ancestor_record_offset":1728,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_direct] utils::components::transform::Transform::compute_matrix
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/components/transform.rs
//     evidence={"module":3619,"record_offset":5556,"parent_record_offset":1728,"ancestor_record_offset":1728,"inlinee":27549,"source_line":23,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3619,"record_offset":4764,"parent_record_offset":4740,"ancestor_record_offset":1728,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3619,"record_offset":4788,"parent_record_offset":4764,"ancestor_record_offset":1728,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3619,"record_offset":5508,"parent_record_offset":5484,"ancestor_record_offset":1728,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3619,"record_offset":4708,"parent_record_offset":4676,"ancestor_record_offset":1728,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3619,"record_offset":4740,"parent_record_offset":4708,"ancestor_record_offset":1728,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3619,"record_offset":5484,"parent_record_offset":5428,"ancestor_record_offset":1728,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":3619,"record_offset":4676,"parent_record_offset":4644,"ancestor_record_offset":1728,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":3619,"record_offset":3708,"parent_record_offset":3684,"ancestor_record_offset":1728,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3619,"record_offset":5456,"parent_record_offset":5428,"ancestor_record_offset":1728,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3619,"record_offset":5428,"parent_record_offset":5404,"ancestor_record_offset":1728,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3619,"record_offset":3628,"parent_record_offset":3604,"ancestor_record_offset":1728,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$49::set_changed
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3619,"record_offset":2132,"parent_record_offset":2108,"ancestor_record_offset":1728,"inlinee":7129,"source_line":309,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3619,"record_offset":3228,"parent_record_offset":3204,"ancestor_record_offset":1728,"inlinee":14620,"source_line":948,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$31::fetch
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3619,"record_offset":2524,"parent_record_offset":2500,"ancestor_record_offset":1728,"inlinee":23287,"source_line":1305,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3619,"record_offset":1960,"parent_record_offset":1936,"ancestor_record_offset":1728,"inlinee":14609,"source_line":36,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3619,"record_offset":1984,"parent_record_offset":1960,"ancestor_record_offset":1728,"inlinee":14611,"source_line":1520,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3619,"record_offset":2220,"parent_record_offset":2196,"ancestor_record_offset":1728,"inlinee":14614,"source_line":1587,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::iter_unchecked_manual
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":3619,"record_offset":1936,"parent_record_offset":1908,"ancestor_record_offset":1728,"inlinee":14607,"source_line":1240,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$4::fetch
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":3619,"record_offset":2500,"parent_record_offset":2220,"ancestor_record_offset":1728,"inlinee":14639,"source_line":201,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$4::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":3619,"record_offset":3204,"parent_record_offset":2912,"ancestor_record_offset":1728,"inlinee":14640,"source_line":192,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":3619,"record_offset":3276,"parent_record_offset":3252,"ancestor_record_offset":1728,"inlinee":14643,"source_line":410,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_data_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3619,"record_offset":3252,"parent_record_offset":3228,"ancestor_record_offset":1728,"inlinee":14642,"source_line":321,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::entities
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3619,"record_offset":2472,"parent_record_offset":2220,"ancestor_record_offset":1728,"inlinee":14616,"source_line":568,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3619,"record_offset":2548,"parent_record_offset":2524,"ancestor_record_offset":1728,"inlinee":14629,"source_line":125,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3619,"record_offset":2388,"parent_record_offset":2220,"ancestor_record_offset":1728,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":3619,"record_offset":5796,"parent_record_offset":5704,"ancestor_record_offset":1728,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3619,"record_offset":5096,"parent_record_offset":4892,"ancestor_record_offset":1728,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3619,"record_offset":4984,"parent_record_offset":4892,"ancestor_record_offset":1728,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3619,"record_offset":6048,"parent_record_offset":6020,"ancestor_record_offset":1728,"inlinee":21867,"source_line":905,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3619,"record_offset":4956,"parent_record_offset":4892,"ancestor_record_offset":1728,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3619,"record_offset":3976,"parent_record_offset":3952,"ancestor_record_offset":1728,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3619,"record_offset":3472,"parent_record_offset":3432,"ancestor_record_offset":1728,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3619,"record_offset":3684,"parent_record_offset":3660,"ancestor_record_offset":1728,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3619,"record_offset":5380,"parent_record_offset":5356,"ancestor_record_offset":1728,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3619,"record_offset":2056,"parent_record_offset":2032,"ancestor_record_offset":1728,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3619,"record_offset":2352,"parent_record_offset":2328,"ancestor_record_offset":1728,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3619,"record_offset":2300,"parent_record_offset":2264,"ancestor_record_offset":1728,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3619,"record_offset":4816,"parent_record_offset":4788,"ancestor_record_offset":1728,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3619,"record_offset":2412,"parent_record_offset":2388,"ancestor_record_offset":1728,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3619,"record_offset":2008,"parent_record_offset":1984,"ancestor_record_offset":1728,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3619,"record_offset":2436,"parent_record_offset":2412,"ancestor_record_offset":1728,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":3619,"record_offset":2032,"parent_record_offset":2008,"ancestor_record_offset":1728,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3619,"record_offset":2328,"parent_record_offset":2264,"ancestor_record_offset":1728,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3619,"record_offset":2264,"parent_record_offset":2220,"ancestor_record_offset":1728,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sqrt
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/math.rs
//     evidence={"module":3619,"record_offset":4328,"parent_record_offset":4224,"ancestor_record_offset":1728,"inlinee":22228,"source_line":187,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::Mat4::from_scale_rotation_translation
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat4.rs
//     evidence={"module":3619,"record_offset":5592,"parent_record_offset":5556,"ancestor_record_offset":1728,"inlinee":27550,"source_line":215,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::Mat4::quat_to_axes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat4.rs
//     evidence={"module":3619,"record_offset":5704,"parent_record_offset":5592,"ancestor_record_offset":1728,"inlinee":27551,"source_line":181,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_rotation_axes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":3619,"record_offset":4224,"parent_record_offset":4008,"ancestor_record_offset":1728,"inlinee":26368,"source_line":192,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_quat
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":3619,"record_offset":4892,"parent_record_offset":4868,"ancestor_record_offset":1728,"inlinee":22239,"source_line":675,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::impl$20::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":3619,"record_offset":5824,"parent_record_offset":5796,"ancestor_record_offset":1728,"inlinee":27553,"source_line":963,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::Vec4::new
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec4.rs
//     evidence={"module":3619,"record_offset":5672,"parent_record_offset":5648,"ancestor_record_offset":1728,"inlinee":14703,"source_line":92,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::Vec4::truncate
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec4.rs
//     evidence={"module":3619,"record_offset":4044,"parent_record_offset":4008,"ancestor_record_offset":1728,"inlinee":23726,"source_line":164,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$10::mul
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec4.rs
//     evidence={"module":3619,"record_offset":6020,"parent_record_offset":5592,"ancestor_record_offset":1728,"inlinee":21866,"source_line":975,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$13::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec4.rs
//     evidence={"module":3619,"record_offset":6884,"parent_record_offset":6612,"ancestor_record_offset":1728,"inlinee":15701,"source_line":998,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$44::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec4.rs
//     evidence={"module":3619,"record_offset":5880,"parent_record_offset":5852,"ancestor_record_offset":1728,"inlinee":27555,"source_line":1273,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$47::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec4.rs
//     evidence={"module":3619,"record_offset":5648,"parent_record_offset":5592,"ancestor_record_offset":1728,"inlinee":27557,"source_line":1300,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3619,"record_offset":3500,"parent_record_offset":3432,"ancestor_record_offset":1728,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3619,"record_offset":3432,"parent_record_offset":3408,"ancestor_record_offset":1728,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] glam::swizzles::sse2::vec4_impl::impl$0::xyz
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/swizzles/sse2/vec4_impl.rs
//     evidence={"module":3619,"record_offset":4068,"parent_record_offset":4044,"ancestor_record_offset":1728,"inlinee":21868,"source_line":223,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":3619,"record_offset":3536,"parent_record_offset":3500,"ancestor_record_offset":1728,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_critters::ui_pet_ducks::ui_pet_ducks::closure$0
// 记录形式: compiled_procedure; instances=1
// 源码行: 121
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3619,"record_offset":7840,"rva":28557600,"va":5397266720}
// 编译实例 RVAs: 0x1b3c120
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: ui_pet_ducks.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24517090","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397266720,"raw_hex":"e81ee689fe","size":5,"soft_fail":false,"target_va":5372749696,"va":5397266781,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: ui_pet_ducks.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24517453","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397266720,"raw_hex":"e8b3e489fe","size":5,"soft_fail":false,"target_va":5372749712,"va":5397267160,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t5711048","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397266720,"raw_hex":"e8c8245700","size":5,"soft_fail":false,"target_va":5402978144,"va":5397267091,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::animation_duration
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21910205","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397266720,"raw_hex":"e843adb1fe","size":5,"soft_fail":false,"target_va":5375356704,"va":5397266904,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21910135","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397266720,"raw_hex":"e889adb1fe","size":5,"soft_fail":false,"target_va":5375356896,"va":5397267026,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21910909","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397266720,"raw_hex":"e883aab1fe","size":5,"soft_fail":false,"target_va":5375355984,"va":5397266888,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::position
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21910390","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397266720,"raw_hex":"e88aacb1fe","size":5,"soft_fail":false,"target_va":5375356560,"va":5397266945,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::rotation
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21910373","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397266720,"raw_hex":"e89bacb1fe","size":5,"soft_fail":false,"target_va":5375356608,"va":5397266976,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::scale
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21910344","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397266720,"raw_hex":"e8b8acb1fe","size":5,"soft_fail":false,"target_va":5375356656,"va":5397266995,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3619,"record_offset":7992,"parent_record_offset":7840,"ancestor_record_offset":7840,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3619,"record_offset":8696,"parent_record_offset":7840,"ancestor_record_offset":7840,"inlinee":4441,"source_line":461,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":3619,"record_offset":8140,"parent_record_offset":7840,"ancestor_record_offset":7840,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3619,"record_offset":8780,"parent_record_offset":7840,"ancestor_record_offset":7840,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3619,"record_offset":8048,"parent_record_offset":8024,"ancestor_record_offset":7840,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3619,"record_offset":8072,"parent_record_offset":8048,"ancestor_record_offset":7840,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3619,"record_offset":8852,"parent_record_offset":8828,"ancestor_record_offset":7840,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3619,"record_offset":8024,"parent_record_offset":7992,"ancestor_record_offset":7840,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3619,"record_offset":8828,"parent_record_offset":8804,"ancestor_record_offset":7840,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":3619,"record_offset":8804,"parent_record_offset":8780,"ancestor_record_offset":7840,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3619,"record_offset":8428,"parent_record_offset":8400,"ancestor_record_offset":7840,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3619,"record_offset":8400,"parent_record_offset":8372,"ancestor_record_offset":7840,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3619,"record_offset":8572,"parent_record_offset":8400,"ancestor_record_offset":7840,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3619,"record_offset":8336,"parent_record_offset":8308,"ancestor_record_offset":7840,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3619,"record_offset":8720,"parent_record_offset":8696,"ancestor_record_offset":7840,"inlinee":4442,"source_line":100,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3619,"record_offset":8308,"parent_record_offset":8280,"ancestor_record_offset":7840,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3619,"record_offset":8252,"parent_record_offset":8224,"ancestor_record_offset":7840,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3619,"record_offset":8280,"parent_record_offset":8252,"ancestor_record_offset":7840,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3619,"record_offset":8224,"parent_record_offset":8196,"ancestor_record_offset":7840,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":3619,"record_offset":8196,"parent_record_offset":8168,"ancestor_record_offset":7840,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":3619,"record_offset":8168,"parent_record_offset":8140,"ancestor_record_offset":7840,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3619,"record_offset":8744,"parent_record_offset":8720,"ancestor_record_offset":7840,"inlinee":4443,"source_line":819,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3619,"record_offset":8372,"parent_record_offset":8336,"ancestor_record_offset":7840,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":3619,"record_offset":8632,"parent_record_offset":8608,"ancestor_record_offset":7840,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3619,"record_offset":8608,"parent_record_offset":8336,"ancestor_record_offset":7840,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3619,"record_offset":8096,"parent_record_offset":8072,"ancestor_record_offset":7840,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_critters::ui_pet_ducks::ui_pet_ducks::closure$1
// 记录形式: compiled_procedure; instances=1
// 源码行: 148
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3619,"record_offset":8896,"rva":28558064,"va":5397267184}
// 编译实例 RVAs: 0x1b3c2f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: ui_pet_ducks.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24517554","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397267184,"raw_hex":"e84ee489fe","size":5,"soft_fail":false,"target_va":5372749696,"va":5397267245,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: ui_pet_ducks.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24518061","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397267184,"raw_hex":"e853e289fe","size":5,"soft_fail":false,"target_va":5372749712,"va":5397267768,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t5710446","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397267184,"raw_hex":"e86e225700","size":5,"soft_fail":false,"target_va":5402978144,"va":5397267693,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t5651515","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397267184,"raw_hex":"e83b3c5600","size":5,"soft_fail":false,"target_va":5402919072,"va":5397267552,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::world::command_queue::CommandQueue::get_raw
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20021044","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397267184,"raw_hex":"e8cc80cefe","size":5,"soft_fail":false,"target_va":5377246528,"va":5397267567,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::animation_duration
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21910680","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397267184,"raw_hex":"e868abb1fe","size":5,"soft_fail":false,"target_va":5375356704,"va":5397267379,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::animation_easing
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21910624","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397267184,"raw_hex":"e8a0abb1fe","size":5,"soft_fail":false,"target_va":5375356800,"va":5397267419,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21910615","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397267184,"raw_hex":"e8a9abb1fe","size":5,"soft_fail":false,"target_va":5375356896,"va":5397267506,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::minimum_lifespan
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21910653","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397267184,"raw_hex":"e883abb1fe","size":5,"soft_fail":false,"target_va":5375356752,"va":5397267400,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21911379","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397267184,"raw_hex":"e8ada8b1fe","size":5,"soft_fail":false,"target_va":5375355984,"va":5397267358,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::position
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21910920","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397267184,"raw_hex":"e878aab1fe","size":5,"soft_fail":false,"target_va":5375356560,"va":5397267475,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::set_ui_pass
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21911168","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397267184,"raw_hex":"e880a9b1fe","size":5,"soft_fail":false,"target_va":5375356272,"va":5397267435,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3619,"record_offset":9052,"parent_record_offset":8896,"ancestor_record_offset":8896,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3619,"record_offset":10312,"parent_record_offset":8896,"ancestor_record_offset":8896,"inlinee":4441,"source_line":461,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::EntityCommands::insert
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3619,"record_offset":9756,"parent_record_offset":8896,"ancestor_record_offset":8896,"inlinee":14636,"source_line":945,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":3619,"record_offset":9200,"parent_record_offset":8896,"ancestor_record_offset":8896,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3619,"record_offset":10396,"parent_record_offset":8896,"ancestor_record_offset":8896,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3619,"record_offset":9108,"parent_record_offset":9084,"ancestor_record_offset":8896,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3619,"record_offset":9132,"parent_record_offset":9108,"ancestor_record_offset":8896,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3619,"record_offset":10468,"parent_record_offset":10444,"ancestor_record_offset":8896,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3619,"record_offset":9084,"parent_record_offset":9052,"ancestor_record_offset":8896,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3619,"record_offset":10444,"parent_record_offset":10420,"ancestor_record_offset":8896,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":3619,"record_offset":10420,"parent_record_offset":10396,"ancestor_record_offset":8896,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3619,"record_offset":9984,"parent_record_offset":9956,"ancestor_record_offset":8896,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3619,"record_offset":9956,"parent_record_offset":9924,"ancestor_record_offset":8896,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3619,"record_offset":9924,"parent_record_offset":9896,"ancestor_record_offset":8896,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3619,"record_offset":9488,"parent_record_offset":9460,"ancestor_record_offset":8896,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3619,"record_offset":9460,"parent_record_offset":9432,"ancestor_record_offset":8896,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3619,"record_offset":9632,"parent_record_offset":9460,"ancestor_record_offset":8896,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3619,"record_offset":9396,"parent_record_offset":9368,"ancestor_record_offset":8896,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3619,"record_offset":10336,"parent_record_offset":10312,"ancestor_record_offset":8896,"inlinee":4442,"source_line":100,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3619,"record_offset":9368,"parent_record_offset":9340,"ancestor_record_offset":8896,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3619,"record_offset":9312,"parent_record_offset":9284,"ancestor_record_offset":8896,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3619,"record_offset":9340,"parent_record_offset":9312,"ancestor_record_offset":8896,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3619,"record_offset":9284,"parent_record_offset":9256,"ancestor_record_offset":8896,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":3619,"record_offset":9256,"parent_record_offset":9228,"ancestor_record_offset":8896,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":3619,"record_offset":9228,"parent_record_offset":9200,"ancestor_record_offset":8896,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3619,"record_offset":10360,"parent_record_offset":10336,"ancestor_record_offset":8896,"inlinee":4443,"source_line":819,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3619,"record_offset":10036,"parent_record_offset":9868,"ancestor_record_offset":8896,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3619,"record_offset":9896,"parent_record_offset":9868,"ancestor_record_offset":8896,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3619,"record_offset":9432,"parent_record_offset":9396,"ancestor_record_offset":8896,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::add
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3619,"record_offset":9804,"parent_record_offset":9780,"ancestor_record_offset":8896,"inlinee":14638,"source_line":748,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::push
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3619,"record_offset":9832,"parent_record_offset":9804,"ancestor_record_offset":8896,"inlinee":12140,"source_line":501,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::add
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3619,"record_offset":9780,"parent_record_offset":9756,"ancestor_record_offset":8896,"inlinee":14637,"source_line":1100,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::CommandQueue::push
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":3619,"record_offset":10068,"parent_record_offset":9832,"ancestor_record_offset":8896,"inlinee":12149,"source_line":78,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::RawCommandQueue::push
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":3619,"record_offset":9868,"parent_record_offset":9832,"ancestor_record_offset":8896,"inlinee":12142,"source_line":157,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":3619,"record_offset":9692,"parent_record_offset":9668,"ancestor_record_offset":8896,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3619,"record_offset":9668,"parent_record_offset":9396,"ancestor_record_offset":8896,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/03619.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3619,"record_offset":9156,"parent_record_offset":9132,"ancestor_record_offset":8896,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/03619.jsonl.gz"}
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
// 完整结构及原始类型记录: ui_pet_ducks.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: ui_pet_ducks.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: ui_pet_ducks.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::critters::Duck>,ref$<utils::components::transform::Transform>,ref$<enum2$<country_core::resources::critters::DuckState> > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::system_param::Local<system_critters::sheep::ui_pet_sheep::IntersectionDepth>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::critters::Duck>,ref$<utils::components::transform::Transform>,ref$<enum2$<country_core::resources::critters::DuckState> > >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::system::system_param::Local<system_critters::sheep::ui_pet_sheep::IntersectionDepth>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::critters::Duck>,ref$<utils::components::transform::Transform>,ref$<enum2$<country_core::resources::critters::DuckState> > >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<system_critters::sheep::ui_pet_sheep::IntersectionDepth>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>

// ------------------------------------------------------------------
// 函数: system_critters::ui_pet_ducks::ui_pet_ducks
// 信息 ID: 497dc4015e23f263dc7fc7fe
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 62240a2647ac24cd27fb5433: void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::critters::Duck>,ref$<utils::components::transform::Transform>,ref$<enum2$<country_core::resources::critters::DuckState> > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::system_param::Local<system_critters::sheep::ui_pet_sheep::IntersectionDepth>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::critters::Duck>,ref$<utils::components::transform::Transform>,ref$<enum2$<country_core::resources::critters::DuckState> > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::system_param::Local<system_critters::sheep::ui_pet_sheep::IntersectionDepth>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::critters::Duck>,ref$<utils::components::transform::Transform>,ref$<enum2$<country_core::resources::critters::DuckState> > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::system_param::Local<system_critters::sheep::ui_pet_sheep::IntersectionDepth>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>)> > @ 0x158648e -> 0x1b3b020 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::critters::Duck>,ref$<utils::components::transform::Transform>,ref$<enum2$<country_core::resources::critters::DuckState> > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::system_param::Local<system_critters::sheep::ui_pet_sheep::IntersectionDepth>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::critters::Duck>,ref$<utils::components::transform::Transform>,ref$<enum2$<country_core::resources::critters::DuckState> > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::system_param::Local<system_critters::sheep::ui_pet_sheep::IntersectionDepth>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>)> @ 0x1591cde -> 0x1b3b020 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::ui_pet_ducks::ui_pet_ducks::closure$0
// 信息 ID: 90d210e651425410b57a3402
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_critters::ui_pet_ducks::ui_pet_ducks", "captures": "unknown"}
// [闭包环境类型引用] system_critters::ui_pet_ducks::ui_pet_ducks::closure_env$0 <- core::ops::function::FnOnce::call_once<system_critters::ui_pet_ducks::ui_pet_ducks::closure_env$0,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_critters::ui_pet_ducks::ui_pet_ducks::closure_env$0 <- core::ptr::drop_in_place<system_critters::ui_pet_ducks::ui_pet_ducks::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 2 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<system_critters::ui_pet_ducks::ui_pet_ducks::closure_env$0,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > | core::ops::function::FnOnce::call_once<system_critters::sheep::ui_pet_sheep::ui_pet_sheep::closure_env$0,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0x1b3af72 -> 0x1b3c120 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::ui_pet_ducks::ui_pet_ducks::closure$1
// 信息 ID: a54f4c3ddd02e6164afcdf83
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_critters::ui_pet_ducks::ui_pet_ducks", "captures": "unknown"}
// [闭包环境类型引用] system_critters::ui_pet_ducks::ui_pet_ducks::closure_env$1 <- core::ops::function::FnOnce::call_once<system_critters::ui_pet_ducks::ui_pet_ducks::closure_env$1,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<system_critters::ui_pet_ducks::ui_pet_ducks::closure_env$1,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0x1b3af92 -> 0x1b3c2f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
