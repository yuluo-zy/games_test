// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/critters/src/ui_pet_ducks.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 5e4d89b04a9bb659461be09f5806824c5564af864e00a9a992658cdd588dc6a4
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: ui_pet_ducks.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 3891
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use bevy_ptr; // namespace candidate
//   package identities: [{"name":"bevy_ptr","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.5"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_critters::ui_pet_ducks::ui_pet_ducks
// 记录形式: compiled_procedure; instances=1
// 源码行: 29
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3891,"record_offset":1080,"rva":36843056,"va":5405552176}
// 编译实例 RVAs: 0x2322e30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: ui_pet_ducks.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-30473762","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405552176,"raw_hex":"e8de012ffe","size":5,"soft_fail":false,"target_va":5375081488,"va":5405555245,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: ui_pet_ducks.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-30474043","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405552176,"raw_hex":"e8c5002ffe","size":5,"soft_fail":false,"target_va":5375081504,"va":5405555542,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140895470
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_pet_ducks.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27847786","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405552176,"raw_hex":"e8961357fe","size":5,"soft_fail":false,"target_va":5377709168,"va":5405556949,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14092df10
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: ui_pet_ducks.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27221075","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405552176,"raw_hex":"e8ada360fe","size":5,"soft_fail":false,"target_va":5378334480,"va":5405555550,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14123b320
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_pet_ducks.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17728512","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405552176,"raw_hex":"e8007cf1fe","size":5,"soft_fail":false,"target_va":5387825952,"va":5405554459,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t5984661","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405552176,"raw_hex":"e895515b00","size":5,"soft_fail":false,"target_va":5411540992,"va":5405556326,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-22714111","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405552176,"raw_hex":"e80169a5fe","size":5,"soft_fail":false,"target_va":5382840208,"va":5405554314,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::audio_resources::AudioInstance::start_if_not_playing
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-26001331","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405552176,"raw_hex":"e84d4073fe","size":5,"soft_fail":false,"target_va":5379552000,"va":5405553326,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::camera_rig::MainCamera::eye_position
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-27334646","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405552176,"raw_hex":"e80ae85efe","size":5,"soft_fail":false,"target_va":5378219472,"va":5405554113,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::camera_rig::MainCamera::mode
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-27336597","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405552176,"raw_hex":"e86be05efe","size":5,"soft_fail":false,"target_va":5378216496,"va":5405553088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::ui_state::UiWidgetContext::intersecting
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-27218163","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405552176,"raw_hex":"e80daf60fe","size":5,"soft_fail":false,"target_va":5378336320,"va":5405554478,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::ui_state::UiWidgetContext::is_active
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-27218120","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405552176,"raw_hex":"e838af60fe","size":5,"soft_fail":false,"target_va":5378336384,"va":5405554499,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::UiSounds::empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-26715944","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405552176,"raw_hex":"e8d85868fe","size":5,"soft_fail":false,"target_va":5378838448,"va":5405554387,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::WidgetParams::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-26716345","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405552176,"raw_hex":"e8475768fe","size":5,"soft_fail":false,"target_va":5378838032,"va":5405554372,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::WidgetParams::no_history_recording
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-26716147","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405552176,"raw_hex":"e80d5868fe","size":5,"soft_fail":false,"target_va":5378838288,"va":5405554430,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::WidgetParams::sounds
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-26716273","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405552176,"raw_hex":"e88f5768fe","size":5,"soft_fail":false,"target_va":5378838144,"va":5405554412,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierFactory::new
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-27812436","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405552176,"raw_hex":"e8ac9d57fe","size":5,"soft_fail":false,"target_va":5377743024,"va":5405555455,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::write
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25127845","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405552176,"raw_hex":"e85b9480fe","size":5,"soft_fail":false,"target_va":5380427664,"va":5405555504,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":3891,"record_offset":8844,"parent_record_offset":1080,"ancestor_record_offset":1080,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":3891,"record_offset":9728,"parent_record_offset":1080,"ancestor_record_offset":1080,"inlinee":4783,"source_line":386,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3891,"record_offset":1660,"parent_record_offset":1080,"ancestor_record_offset":1080,"inlinee":9177,"source_line":434,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3891,"record_offset":1764,"parent_record_offset":1080,"ancestor_record_offset":1080,"inlinee":11673,"source_line":884,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3891,"record_offset":1256,"parent_record_offset":1080,"ancestor_record_offset":1080,"inlinee":17020,"source_line":701,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":3891,"record_offset":1724,"parent_record_offset":1080,"ancestor_record_offset":1080,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hint.rs
//     evidence={"module":3891,"record_offset":9700,"parent_record_offset":1080,"ancestor_record_offset":1080,"inlinee":7064,"source_line":599,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3891,"record_offset":10456,"parent_record_offset":1080,"ancestor_record_offset":1080,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::mat4::Mat4::transform_point3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat4.rs
//     evidence={"module":3891,"record_offset":11768,"parent_record_offset":1080,"ancestor_record_offset":1080,"inlinee":29550,"source_line":1151,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::from_mat4
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":3891,"record_offset":9136,"parent_record_offset":1080,"ancestor_record_offset":1080,"inlinee":122555,"source_line":296,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::impl$7::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":3891,"record_offset":9952,"parent_record_offset":1080,"ancestor_record_offset":1080,"inlinee":13688,"source_line":894,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3891,"record_offset":8648,"parent_record_offset":1080,"ancestor_record_offset":1080,"inlinee":13631,"source_line":477,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_direct] utils::components::transform::Transform::compute_matrix
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/components/transform.rs
//     evidence={"module":3891,"record_offset":10684,"parent_record_offset":1080,"ancestor_record_offset":1080,"inlinee":25133,"source_line":24,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3891,"record_offset":9848,"parent_record_offset":9824,"ancestor_record_offset":1080,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3891,"record_offset":9872,"parent_record_offset":9848,"ancestor_record_offset":1080,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3891,"record_offset":10632,"parent_record_offset":10608,"ancestor_record_offset":1080,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3891,"record_offset":9792,"parent_record_offset":9760,"ancestor_record_offset":1080,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3891,"record_offset":9824,"parent_record_offset":9792,"ancestor_record_offset":1080,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3891,"record_offset":10608,"parent_record_offset":10552,"ancestor_record_offset":1080,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":3891,"record_offset":9760,"parent_record_offset":9728,"ancestor_record_offset":1080,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":3891,"record_offset":8892,"parent_record_offset":8868,"ancestor_record_offset":1080,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3891,"record_offset":1492,"parent_record_offset":1468,"ancestor_record_offset":1080,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3891,"record_offset":10580,"parent_record_offset":10552,"ancestor_record_offset":1080,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3891,"record_offset":10552,"parent_record_offset":10528,"ancestor_record_offset":1080,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3891,"record_offset":1540,"parent_record_offset":1516,"ancestor_record_offset":1080,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3891,"record_offset":1516,"parent_record_offset":1492,"ancestor_record_offset":1080,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3891,"record_offset":10528,"parent_record_offset":10504,"ancestor_record_offset":1080,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3891,"record_offset":1468,"parent_record_offset":1444,"ancestor_record_offset":1080,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3891,"record_offset":1444,"parent_record_offset":1412,"ancestor_record_offset":1080,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3891,"record_offset":2064,"parent_record_offset":2040,"ancestor_record_offset":1080,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3891,"record_offset":2088,"parent_record_offset":2064,"ancestor_record_offset":1080,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3891,"record_offset":1412,"parent_record_offset":1388,"ancestor_record_offset":1080,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::entities
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3891,"record_offset":3648,"parent_record_offset":1796,"ancestor_record_offset":1080,"inlinee":11682,"source_line":466,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::is_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3891,"record_offset":2952,"parent_record_offset":1796,"ancestor_record_offset":1080,"inlinee":11679,"source_line":615,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::table_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3891,"record_offset":3036,"parent_record_offset":1796,"ancestor_record_offset":1080,"inlinee":5495,"source_line":460,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeEntity::table_row
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3891,"record_offset":3844,"parent_record_offset":1796,"ancestor_record_offset":1080,"inlinee":17012,"source_line":325,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetypes::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3891,"record_offset":2868,"parent_record_offset":1796,"ancestor_record_offset":1080,"inlinee":11678,"source_line":895,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3891,"record_offset":1688,"parent_record_offset":1660,"ancestor_record_offset":1080,"inlinee":9179,"source_line":397,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3891,"record_offset":2664,"parent_record_offset":2640,"ancestor_record_offset":1080,"inlinee":4876,"source_line":2472,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_archetype
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3891,"record_offset":6444,"parent_record_offset":6304,"ancestor_record_offset":1080,"inlinee":4856,"source_line":1160,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_table
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3891,"record_offset":5260,"parent_record_offset":5148,"ancestor_record_offset":1080,"inlinee":4857,"source_line":1175,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3891,"record_offset":8432,"parent_record_offset":8408,"ancestor_record_offset":1080,"inlinee":4878,"source_line":1235,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3891,"record_offset":3152,"parent_record_offset":3128,"ancestor_record_offset":1080,"inlinee":13655,"source_line":1524,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3891,"record_offset":2148,"parent_record_offset":2124,"ancestor_record_offset":1080,"inlinee":13656,"source_line":1539,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3891,"record_offset":2640,"parent_record_offset":2616,"ancestor_record_offset":1080,"inlinee":13686,"source_line":1596,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3891,"record_offset":2688,"parent_record_offset":2664,"ancestor_record_offset":1080,"inlinee":9533,"source_line":1602,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$65::fetch
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3891,"record_offset":2616,"parent_record_offset":1796,"ancestor_record_offset":1080,"inlinee":17024,"source_line":2083,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3891,"record_offset":1364,"parent_record_offset":1280,"ancestor_record_offset":1080,"inlinee":13709,"source_line":39,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3891,"record_offset":1388,"parent_record_offset":1364,"ancestor_record_offset":1080,"inlinee":9292,"source_line":2359,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3891,"record_offset":1796,"parent_record_offset":1764,"ancestor_record_offset":1080,"inlinee":11674,"source_line":2458,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$4::set_archetype
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":3891,"record_offset":3128,"parent_record_offset":1796,"ancestor_record_offset":1080,"inlinee":17025,"source_line":182,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$4::set_table
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":3891,"record_offset":2124,"parent_record_offset":1796,"ancestor_record_offset":1080,"inlinee":17022,"source_line":195,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs
//     evidence={"module":3891,"record_offset":2520,"parent_record_offset":2496,"ancestor_record_offset":1080,"inlinee":4872,"source_line":143,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3891,"record_offset":2228,"parent_record_offset":2204,"ancestor_record_offset":1080,"inlinee":4864,"source_line":58,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3891,"record_offset":2204,"parent_record_offset":2180,"ancestor_record_offset":1080,"inlinee":4862,"source_line":395,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3891,"record_offset":2364,"parent_record_offset":2340,"ancestor_record_offset":1080,"inlinee":4866,"source_line":396,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3891,"record_offset":7456,"parent_record_offset":4568,"ancestor_record_offset":1080,"inlinee":11681,"source_line":211,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3891,"record_offset":2180,"parent_record_offset":2148,"ancestor_record_offset":1080,"inlinee":4860,"source_line":475,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3891,"record_offset":5284,"parent_record_offset":5260,"ancestor_record_offset":1080,"inlinee":4859,"source_line":363,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::is_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3891,"record_offset":2040,"parent_record_offset":1796,"ancestor_record_offset":1080,"inlinee":11675,"source_line":615,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3891,"record_offset":2712,"parent_record_offset":2688,"ancestor_record_offset":1080,"inlinee":5472,"source_line":127,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3891,"record_offset":1956,"parent_record_offset":1796,"ancestor_record_offset":1080,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_changed_ticks_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":3891,"record_offset":2552,"parent_record_offset":2148,"ancestor_record_offset":1080,"inlinee":13683,"source_line":319,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":3891,"record_offset":2496,"parent_record_offset":2148,"ancestor_record_offset":1080,"inlinee":4870,"source_line":303,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3891,"record_offset":5624,"parent_record_offset":5600,"ancestor_record_offset":1080,"inlinee":4868,"source_line":368,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::thin_array_ptr::ThinArrayPtr::as_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/thin_array_ptr.rs
//     evidence={"module":3891,"record_offset":2576,"parent_record_offset":2552,"ancestor_record_offset":1080,"inlinee":13684,"source_line":300,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::copy_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3891,"record_offset":1332,"parent_record_offset":1308,"ancestor_record_offset":1080,"inlinee":13707,"source_line":636,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3891,"record_offset":1280,"parent_record_offset":1256,"ancestor_record_offset":1080,"inlinee":17021,"source_line":601,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3891,"record_offset":1308,"parent_record_offset":1280,"ancestor_record_offset":1080,"inlinee":13706,"source_line":618,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::impl$4::into_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3891,"record_offset":4164,"parent_record_offset":4140,"ancestor_record_offset":1080,"inlinee":25723,"source_line":2574,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ptr","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs
//     evidence={"module":3891,"record_offset":3996,"parent_record_offset":3972,"ancestor_record_offset":1080,"inlinee":4880,"source_line":526,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":3891,"record_offset":10924,"parent_record_offset":10832,"ancestor_record_offset":1080,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3891,"record_offset":10188,"parent_record_offset":9976,"ancestor_record_offset":1080,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3891,"record_offset":10072,"parent_record_offset":9976,"ancestor_record_offset":1080,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3891,"record_offset":11176,"parent_record_offset":11148,"ancestor_record_offset":1080,"inlinee":24651,"source_line":910,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3891,"record_offset":10040,"parent_record_offset":9976,"ancestor_record_offset":1080,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3891,"record_offset":8712,"parent_record_offset":8672,"ancestor_record_offset":1080,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":3891,"record_offset":2276,"parent_record_offset":2252,"ancestor_record_offset":1080,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3891,"record_offset":2252,"parent_record_offset":2228,"ancestor_record_offset":1080,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3891,"record_offset":2300,"parent_record_offset":2276,"ancestor_record_offset":1080,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3891,"record_offset":2340,"parent_record_offset":2204,"ancestor_record_offset":1080,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3891,"record_offset":8868,"parent_record_offset":8844,"ancestor_record_offset":1080,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3891,"record_offset":10480,"parent_record_offset":10456,"ancestor_record_offset":1080,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3891,"record_offset":1608,"parent_record_offset":1584,"ancestor_record_offset":1080,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3891,"record_offset":1920,"parent_record_offset":1884,"ancestor_record_offset":1080,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3891,"record_offset":9900,"parent_record_offset":9872,"ancestor_record_offset":1080,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3891,"record_offset":1980,"parent_record_offset":1956,"ancestor_record_offset":1080,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3891,"record_offset":7652,"parent_record_offset":4568,"ancestor_record_offset":1080,"inlinee":5263,"source_line":656,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3891,"record_offset":2444,"parent_record_offset":2420,"ancestor_record_offset":1080,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3891,"record_offset":2004,"parent_record_offset":1980,"ancestor_record_offset":1080,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3891,"record_offset":2420,"parent_record_offset":2388,"ancestor_record_offset":1080,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":3891,"record_offset":1584,"parent_record_offset":1412,"ancestor_record_offset":1080,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3891,"record_offset":1884,"parent_record_offset":1796,"ancestor_record_offset":1080,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sqrt
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/math.rs
//     evidence={"module":3891,"record_offset":9376,"parent_record_offset":9284,"ancestor_record_offset":1080,"inlinee":4753,"source_line":177,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::Mat4::from_scale_rotation_translation
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat4.rs
//     evidence={"module":3891,"record_offset":10720,"parent_record_offset":10684,"ancestor_record_offset":1080,"inlinee":25134,"source_line":218,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::Mat4::quat_to_axes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat4.rs
//     evidence={"module":3891,"record_offset":10832,"parent_record_offset":10720,"ancestor_record_offset":1080,"inlinee":25135,"source_line":184,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_rotation_axes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":3891,"record_offset":9284,"parent_record_offset":9136,"ancestor_record_offset":1080,"inlinee":12703,"source_line":198,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_quat
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":3891,"record_offset":9976,"parent_record_offset":9952,"ancestor_record_offset":1080,"inlinee":13682,"source_line":727,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::impl$20::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":3891,"record_offset":10952,"parent_record_offset":10924,"ancestor_record_offset":1080,"inlinee":25137,"source_line":1011,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::Vec4::new
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec4.rs
//     evidence={"module":3891,"record_offset":10800,"parent_record_offset":10776,"ancestor_record_offset":1080,"inlinee":18181,"source_line":91,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::Vec4::truncate
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec4.rs
//     evidence={"module":3891,"record_offset":9172,"parent_record_offset":9136,"ancestor_record_offset":1080,"inlinee":33089,"source_line":174,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$100::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec4.rs
//     evidence={"module":3891,"record_offset":11008,"parent_record_offset":10980,"ancestor_record_offset":1080,"inlinee":25139,"source_line":1784,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$103::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec4.rs
//     evidence={"module":3891,"record_offset":10776,"parent_record_offset":10720,"ancestor_record_offset":1080,"inlinee":25143,"source_line":1811,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$25::mul
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec4.rs
//     evidence={"module":3891,"record_offset":11148,"parent_record_offset":10720,"ancestor_record_offset":1080,"inlinee":25141,"source_line":1167,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$35::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec4.rs
//     evidence={"module":3891,"record_offset":12040,"parent_record_offset":11768,"ancestor_record_offset":1080,"inlinee":28544,"source_line":1245,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3891,"record_offset":8740,"parent_record_offset":8672,"ancestor_record_offset":1080,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3891,"record_offset":8672,"parent_record_offset":8648,"ancestor_record_offset":1080,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] glam::swizzles::sse2::vec4_impl::impl$0::xyz
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/swizzles/sse2/vec4_impl.rs
//     evidence={"module":3891,"record_offset":9196,"parent_record_offset":9172,"ancestor_record_offset":1080,"inlinee":30322,"source_line":199,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":3891,"record_offset":2388,"parent_record_offset":2364,"ancestor_record_offset":1080,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":3891,"record_offset":8776,"parent_record_offset":8740,"ancestor_record_offset":1080,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/03891.jsonl.gz"}
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
// 首条位置: {"module":3891,"record_offset":13048,"rva":36848272,"va":5405557392}
// 编译实例 RVAs: 0x2324290
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: ui_pet_ducks.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-30475970","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405557392,"raw_hex":"e83ef92efe","size":5,"soft_fail":false,"target_va":5375081488,"va":5405557453,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: ui_pet_ducks.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-30476333","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405557392,"raw_hex":"e8d3f72efe","size":5,"soft_fail":false,"target_va":5375081504,"va":5405557832,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5983224","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405557392,"raw_hex":"e8f84b5b00","size":5,"soft_fail":false,"target_va":5411540992,"va":5405557763,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5983173","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405557392,"raw_hex":"e8c54b5b00","size":5,"soft_fail":false,"target_va":5411540963,"va":5405557785,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::animation_duration
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-27813565","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405557392,"raw_hex":"e8439957fe","size":5,"soft_fail":false,"target_va":5377744016,"va":5405557576,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-27813495","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405557392,"raw_hex":"e8899957fe","size":5,"soft_fail":false,"target_va":5377744208,"va":5405557698,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-27814285","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405557392,"raw_hex":"e8739657fe","size":5,"soft_fail":false,"target_va":5377743280,"va":5405557560,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::position
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-27813750","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405557392,"raw_hex":"e88a9857fe","size":5,"soft_fail":false,"target_va":5377743872,"va":5405557617,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::rotation
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-27813733","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405557392,"raw_hex":"e89b9857fe","size":5,"soft_fail":false,"target_va":5377743920,"va":5405557648,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::scale
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-27813704","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405557392,"raw_hex":"e8b89857fe","size":5,"soft_fail":false,"target_va":5377743968,"va":5405557667,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3891,"record_offset":13200,"parent_record_offset":13048,"ancestor_record_offset":13048,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3891,"record_offset":13884,"parent_record_offset":13048,"ancestor_record_offset":13048,"inlinee":7065,"source_line":526,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":3891,"record_offset":13348,"parent_record_offset":13048,"ancestor_record_offset":13048,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3891,"record_offset":13968,"parent_record_offset":13048,"ancestor_record_offset":13048,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3891,"record_offset":13256,"parent_record_offset":13232,"ancestor_record_offset":13048,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3891,"record_offset":13280,"parent_record_offset":13256,"ancestor_record_offset":13048,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3891,"record_offset":14040,"parent_record_offset":14016,"ancestor_record_offset":13048,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3891,"record_offset":13232,"parent_record_offset":13200,"ancestor_record_offset":13048,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3891,"record_offset":14016,"parent_record_offset":13992,"ancestor_record_offset":13048,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":3891,"record_offset":13992,"parent_record_offset":13968,"ancestor_record_offset":13048,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3891,"record_offset":13608,"parent_record_offset":13580,"ancestor_record_offset":13048,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3891,"record_offset":13668,"parent_record_offset":13636,"ancestor_record_offset":13048,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3891,"record_offset":13636,"parent_record_offset":13608,"ancestor_record_offset":13048,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3891,"record_offset":13544,"parent_record_offset":13516,"ancestor_record_offset":13048,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3891,"record_offset":13908,"parent_record_offset":13884,"ancestor_record_offset":13048,"inlinee":7066,"source_line":99,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3891,"record_offset":13516,"parent_record_offset":13488,"ancestor_record_offset":13048,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3891,"record_offset":13460,"parent_record_offset":13432,"ancestor_record_offset":13048,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3891,"record_offset":13488,"parent_record_offset":13460,"ancestor_record_offset":13048,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3891,"record_offset":13432,"parent_record_offset":13404,"ancestor_record_offset":13048,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":3891,"record_offset":13404,"parent_record_offset":13376,"ancestor_record_offset":13048,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":3891,"record_offset":13376,"parent_record_offset":13348,"ancestor_record_offset":13048,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3891,"record_offset":13932,"parent_record_offset":13908,"ancestor_record_offset":13048,"inlinee":7067,"source_line":934,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3891,"record_offset":13580,"parent_record_offset":13544,"ancestor_record_offset":13048,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":3891,"record_offset":13820,"parent_record_offset":13796,"ancestor_record_offset":13048,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3891,"record_offset":13796,"parent_record_offset":13544,"ancestor_record_offset":13048,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3891,"record_offset":13304,"parent_record_offset":13280,"ancestor_record_offset":13048,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/03891.jsonl.gz"}
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
// 首条位置: {"module":3891,"record_offset":14084,"rva":36848736,"va":5405557856}
// 编译实例 RVAs: 0x2324460
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: ui_pet_ducks.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-30476434","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405557856,"raw_hex":"e86ef72efe","size":5,"soft_fail":false,"target_va":5375081488,"va":5405557917,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: ui_pet_ducks.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-30476925","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405557856,"raw_hex":"e883f52efe","size":5,"soft_fail":false,"target_va":5375081504,"va":5405558424,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5982636","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405557856,"raw_hex":"e8ac495b00","size":5,"soft_fail":false,"target_va":5411540992,"va":5405558351,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5771661","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405557856,"raw_hex":"e88d115800","size":5,"soft_fail":false,"target_va":5411330000,"va":5405558334,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5982585","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405557856,"raw_hex":"e879495b00","size":5,"soft_fail":false,"target_va":5411540963,"va":5405558373,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::animation_duration
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-27814040","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405557856,"raw_hex":"e8689757fe","size":5,"soft_fail":false,"target_va":5377744016,"va":5405558051,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::animation_easing
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-27813984","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405557856,"raw_hex":"e8a09757fe","size":5,"soft_fail":false,"target_va":5377744112,"va":5405558091,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-27813975","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405557856,"raw_hex":"e8a99757fe","size":5,"soft_fail":false,"target_va":5377744208,"va":5405558178,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::minimum_lifespan
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-27814013","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405557856,"raw_hex":"e8839757fe","size":5,"soft_fail":false,"target_va":5377744064,"va":5405558072,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-27814755","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405557856,"raw_hex":"e89d9457fe","size":5,"soft_fail":false,"target_va":5377743280,"va":5405558030,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::position
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-27814280","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405557856,"raw_hex":"e8789657fe","size":5,"soft_fail":false,"target_va":5377743872,"va":5405558147,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::set_ui_pass
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-27814544","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405557856,"raw_hex":"e8709557fe","size":5,"soft_fail":false,"target_va":5377743568,"va":5405558107,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3891,"record_offset":14240,"parent_record_offset":14084,"ancestor_record_offset":14084,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3891,"record_offset":15228,"parent_record_offset":14084,"ancestor_record_offset":14084,"inlinee":7065,"source_line":526,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::EntityCommands::insert
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":3891,"record_offset":14924,"parent_record_offset":14084,"ancestor_record_offset":14084,"inlinee":9427,"source_line":1347,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":3891,"record_offset":14388,"parent_record_offset":14084,"ancestor_record_offset":14084,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3891,"record_offset":15312,"parent_record_offset":14084,"ancestor_record_offset":14084,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3891,"record_offset":14296,"parent_record_offset":14272,"ancestor_record_offset":14084,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3891,"record_offset":14320,"parent_record_offset":14296,"ancestor_record_offset":14084,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3891,"record_offset":15384,"parent_record_offset":15360,"ancestor_record_offset":14084,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3891,"record_offset":14272,"parent_record_offset":14240,"ancestor_record_offset":14084,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3891,"record_offset":15360,"parent_record_offset":15336,"ancestor_record_offset":14084,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":3891,"record_offset":15336,"parent_record_offset":15312,"ancestor_record_offset":14084,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3891,"record_offset":15076,"parent_record_offset":15048,"ancestor_record_offset":14084,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3891,"record_offset":14648,"parent_record_offset":14620,"ancestor_record_offset":14084,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3891,"record_offset":15160,"parent_record_offset":15136,"ancestor_record_offset":14084,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3891,"record_offset":15136,"parent_record_offset":15104,"ancestor_record_offset":14084,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3891,"record_offset":15104,"parent_record_offset":15076,"ancestor_record_offset":14084,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3891,"record_offset":14708,"parent_record_offset":14676,"ancestor_record_offset":14084,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3891,"record_offset":14676,"parent_record_offset":14648,"ancestor_record_offset":14084,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3891,"record_offset":14584,"parent_record_offset":14556,"ancestor_record_offset":14084,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3891,"record_offset":15252,"parent_record_offset":15228,"ancestor_record_offset":14084,"inlinee":7066,"source_line":99,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3891,"record_offset":14556,"parent_record_offset":14528,"ancestor_record_offset":14084,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3891,"record_offset":14500,"parent_record_offset":14472,"ancestor_record_offset":14084,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3891,"record_offset":14528,"parent_record_offset":14500,"ancestor_record_offset":14084,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3891,"record_offset":14472,"parent_record_offset":14444,"ancestor_record_offset":14084,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":3891,"record_offset":14444,"parent_record_offset":14416,"ancestor_record_offset":14084,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":3891,"record_offset":14416,"parent_record_offset":14388,"ancestor_record_offset":14084,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3891,"record_offset":15276,"parent_record_offset":15252,"ancestor_record_offset":14084,"inlinee":7067,"source_line":934,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3891,"record_offset":15048,"parent_record_offset":15012,"ancestor_record_offset":14084,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3891,"record_offset":14620,"parent_record_offset":14584,"ancestor_record_offset":14084,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":3891,"record_offset":14980,"parent_record_offset":14952,"ancestor_record_offset":14084,"inlinee":4959,"source_line":553,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":3891,"record_offset":15012,"parent_record_offset":14980,"ancestor_record_offset":14084,"inlinee":4960,"source_line":612,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":3891,"record_offset":14952,"parent_record_offset":14924,"ancestor_record_offset":14084,"inlinee":9420,"source_line":1798,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":3891,"record_offset":14860,"parent_record_offset":14836,"ancestor_record_offset":14084,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3891,"record_offset":14836,"parent_record_offset":14584,"ancestor_record_offset":14084,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/03891.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3891,"record_offset":14344,"parent_record_offset":14320,"ancestor_record_offset":14084,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/03891.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::critters::Duck>,ref$<utils::components::transform::Transform>,ref$<enum2$<country_core::resources::critters::DuckState> > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::system_param::Local<system_critters::sheep::ui_pet_sheep::IntersectionDepth>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::critters::Duck>,ref$<utils::components::transform::Transform>,ref$<enum2$<country_core::resources::critters::DuckState> > >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::system::system_param::Local<system_critters::sheep::ui_pet_sheep::IntersectionDepth>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::critters::Duck>,ref$<utils::components::transform::Transform>,ref$<enum2$<country_core::resources::critters::DuckState> > >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<system_critters::sheep::ui_pet_sheep::IntersectionDepth>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>

// ------------------------------------------------------------------
// 函数: system_critters::ui_pet_ducks::ui_pet_ducks
// 信息 ID: 358880b02ab4a21d026dd942
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 33a3c0b4d420673eec78cb7c: void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::critters::Duck>,ref$<utils::components::transform::Transform>,ref$<enum2$<country_core::resources::critters::DuckState> > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::system_param::Local<system_critters::sheep::ui_pet_sheep::IntersectionDepth>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::critters::Duck>,ref$<utils::components::transform::Transform>,ref$<enum2$<country_core::resources::critters::DuckState> > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::system_param::Local<system_critters::sheep::ui_pet_sheep::IntersectionDepth>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::critters::Duck>,ref$<utils::components::transform::Transform>,ref$<enum2$<country_core::resources::critters::DuckState> > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::system_param::Local<system_critters::sheep::ui_pet_sheep::IntersectionDepth>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>)> > > @ 0x1da4a92 -> 0x2322e30 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::critters::Duck>,ref$<utils::components::transform::Transform>,ref$<enum2$<country_core::resources::critters::DuckState> > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::system_param::Local<system_critters::sheep::ui_pet_sheep::IntersectionDepth>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::critters::Duck>,ref$<utils::components::transform::Transform>,ref$<enum2$<country_core::resources::critters::DuckState> > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::system_param::Local<system_critters::sheep::ui_pet_sheep::IntersectionDepth>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>)> > @ 0x1da67dd -> 0x2322e30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::ui_pet_ducks::ui_pet_ducks::closure$0
// 信息 ID: 831bd0a38f50136bb0bdf48d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_critters::ui_pet_ducks::ui_pet_ducks", "captures": "unknown"}
// [闭包环境类型引用] system_critters::ui_pet_ducks::ui_pet_ducks::closure_env$0 <- core::ops::function::FnOnce::call_once<system_critters::ui_pet_ducks::ui_pet_ducks::closure_env$0,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<system_critters::ui_pet_ducks::ui_pet_ducks::closure_env$0,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0x2322e22 -> 0x2324290 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::ui_pet_ducks::ui_pet_ducks::closure$1
// 信息 ID: 215b976a4bcb6495dbba8e61
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_critters::ui_pet_ducks::ui_pet_ducks", "captures": "unknown"}
// [闭包环境类型引用] system_critters::ui_pet_ducks::ui_pet_ducks::closure_env$1 <- core::ops::function::FnOnce::call_once<system_critters::ui_pet_ducks::ui_pet_ducks::closure_env$1,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<system_critters::ui_pet_ducks::ui_pet_ducks::closure_env$1,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0x2322e02 -> 0x2324460 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
