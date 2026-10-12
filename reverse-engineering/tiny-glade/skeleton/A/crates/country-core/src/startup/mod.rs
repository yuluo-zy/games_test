// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/startup/mod.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 9b411d2cedab59c7c2b9707837e16153cb5f9a71d9bb851f6e34a28efdaf418f
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: mod.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 396, 398, 419, 420, 435, 448, 454, 469, 471, 473, 475, 477, 480, 486, 492, 497, 1325, 1341, 1365, 1403
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_app; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use shader_pipeline; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::startup::impl$0::add_game_startup_systems
// 记录形式: compiled_procedure; instances=1
// 源码行: 102
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":480,"record_offset":17356,"rva":6796240,"va":5375505360}
// 编译实例 RVAs: 0x67b3d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405454d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1269530","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375505360,"raw_hex":"e8e6a0ecff","size":5,"soft_fail":false,"target_va":5374235856,"va":5375505381,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406b1f70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t224065","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375505360,"raw_hex":"e8416b0300","size":5,"soft_fail":false,"target_va":5375729520,"va":5375505450,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d8ff80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7424875","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375505360,"raw_hex":"e86b4b7100","size":5,"soft_fail":false,"target_va":5382930304,"va":5375505424,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1410899a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10544542","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375505360,"raw_hex":"e89ee5a000","size":5,"soft_fail":false,"target_va":5386049952,"va":5375505405,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_app::sub_app::SubApp::add_systems
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t7531643","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375505360,"raw_hex":"e87bec7200","size":5,"soft_fail":false,"target_va":5383037040,"va":5375505392,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_app::app::App::add_systems
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_app/src/app.rs
//     evidence={"module":480,"record_offset":17500,"parent_record_offset":17356,"ancestor_record_offset":17356,"inlinee":9705,"source_line":284,"archive":"_pdb/A/modules/00480.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::audio::impl$1::add_audio_startup_systems
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/audio/mod.rs
//     evidence={"module":480,"record_offset":17612,"parent_record_offset":17356,"ancestor_record_offset":17356,"inlinee":27117,"source_line":147,"archive":"_pdb/A/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] bevy_app::app::App::add_systems
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_app/src/app.rs
//     evidence={"module":480,"record_offset":17636,"parent_record_offset":17612,"ancestor_record_offset":17356,"inlinee":9705,"source_line":284,"archive":"_pdb/A/modules/00480.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::impl$0::add_game_startup_systems::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 160
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6BED
// 首条位置: {"module":492,"inlinee":27629}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::event::EventWriter::send_default
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":492,"record_offset":4684,"parent_record_offset":4660,"ancestor_record_offset":4660,"inlinee":7452,"source_line":641,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":492,"record_offset":4864,"parent_record_offset":4840,"ancestor_record_offset":4660,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":492,"record_offset":4840,"parent_record_offset":4804,"ancestor_record_offset":4660,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":492,"record_offset":4804,"parent_record_offset":4768,"ancestor_record_offset":4660,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":492,"record_offset":4740,"parent_record_offset":4708,"ancestor_record_offset":4660,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":492,"record_offset":4708,"parent_record_offset":4684,"ancestor_record_offset":4660,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::send
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":492,"record_offset":4768,"parent_record_offset":4708,"ancestor_record_offset":4660,"inlinee":7451,"source_line":231,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":492,"record_offset":4896,"parent_record_offset":4804,"ancestor_record_offset":4660,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::impl$0::add_game_startup_systems::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 170
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6BEC
// 首条位置: {"module":492,"inlinee":27628}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::event::EventWriter::send
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":492,"record_offset":2868,"parent_record_offset":2844,"ancestor_record_offset":2844,"inlinee":7450,"source_line":624,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":492,"record_offset":3052,"parent_record_offset":3020,"ancestor_record_offset":2844,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":492,"record_offset":3020,"parent_record_offset":2984,"ancestor_record_offset":2844,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":492,"record_offset":2984,"parent_record_offset":2948,"ancestor_record_offset":2844,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":492,"record_offset":2916,"parent_record_offset":2892,"ancestor_record_offset":2844,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":492,"record_offset":2892,"parent_record_offset":2868,"ancestor_record_offset":2844,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::send
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":492,"record_offset":2948,"parent_record_offset":2868,"ancestor_record_offset":2844,"inlinee":7451,"source_line":231,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":492,"record_offset":3084,"parent_record_offset":2984,"ancestor_record_offset":2844,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::impl$1::build
// 记录形式: compiled_procedure; instances=1
// 源码行: 197
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":454,"record_offset":3204,"rva":6418096,"va":5375127216}
// 编译实例 RVAs: 0x61eeb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] bevy_app::sub_app::SubApp::add_systems
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tjmp\t7914312","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5375127216,"raw_hex":"e948c37800","size":5,"soft_fail":false,"target_va":5383041536,"va":5375127219,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_app::app::App::add_systems
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_app/src/app.rs
//     evidence={"module":454,"record_offset":3324,"parent_record_offset":3204,"ancestor_record_offset":3204,"inlinee":9705,"source_line":284,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_shader_into_library<alloc::string::String>
// 记录形式: compiled_procedure; instances=1
// 源码行: 210
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":492,"record_offset":16368,"rva":7021472,"va":5375730592}
// 编译实例 RVAs: 0x6b23a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e16c0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6098308","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375730592,"raw_hex":"e87cf2a2ff","size":5,"soft_fail":false,"target_va":5369632448,"va":5375730751,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2980975","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375730592,"raw_hex":"e89183d2ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5375730666,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2981641","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375730592,"raw_hex":"e8f780d2ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5375731348,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140650460
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-401482","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375730592,"raw_hex":"e8b6dff9ff","size":5,"soft_fail":false,"target_va":5375329376,"va":5375730853,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406e8c80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t222636","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375730592,"raw_hex":"e8ac650300","size":5,"soft_fail":false,"target_va":5375954048,"va":5375731407,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27862648","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375730592,"raw_hex":"e87826a901","size":5,"soft_fail":false,"target_va":5403593360,"va":5375730707,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27246852","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375730592,"raw_hex":"e804c19f01","size":5,"soft_fail":false,"target_va":5402978144,"va":5375731287,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1795841","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375730592,"raw_hex":"e801671b00","size":5,"soft_fail":false,"target_va":5377527120,"va":5375731274,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1632860","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375730592,"raw_hex":"e85cea1800","size":5,"soft_fail":false,"target_va":5377364336,"va":5375731471,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27247606","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375730592,"raw_hex":"e8f6c39f01","size":5,"soft_fail":false,"target_va":5402978880,"va":5375731269,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27248868","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375730592,"raw_hex":"e8e4c89f01","size":5,"soft_fail":false,"target_va":5402980128,"va":5375731255,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::shader_library::AssetShaderLibrary::add_raster
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-41941","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375730592,"raw_hex":"e82b5cffff","size":5,"soft_fail":false,"target_va":5375689248,"va":5375731184,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":492,"record_offset":16536,"parent_record_offset":16368,"ancestor_record_offset":16368,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":492,"record_offset":17564,"parent_record_offset":16368,"ancestor_record_offset":16368,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::shader::RasterShaderProgram::new_draw
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/shader.rs
//     evidence={"module":492,"record_offset":17436,"parent_record_offset":16368,"ancestor_record_offset":16368,"inlinee":26623,"source_line":94,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::pipeline_desc::RasterPipelineDesc::new_vertex_fragment
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":492,"record_offset":16592,"parent_record_offset":16368,"ancestor_record_offset":16368,"inlinee":27635,"source_line":24,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":492,"record_offset":17012,"parent_record_offset":16988,"ancestor_record_offset":16368,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":492,"record_offset":17036,"parent_record_offset":17012,"ancestor_record_offset":16368,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":492,"record_offset":17384,"parent_record_offset":17360,"ancestor_record_offset":16368,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":492,"record_offset":16988,"parent_record_offset":16944,"ancestor_record_offset":16368,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":492,"record_offset":17360,"parent_record_offset":17304,"ancestor_record_offset":16368,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":492,"record_offset":17332,"parent_record_offset":17304,"ancestor_record_offset":16368,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":492,"record_offset":16944,"parent_record_offset":16916,"ancestor_record_offset":16368,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":492,"record_offset":16916,"parent_record_offset":16888,"ancestor_record_offset":16368,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":492,"record_offset":17104,"parent_record_offset":16916,"ancestor_record_offset":16368,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":492,"record_offset":17304,"parent_record_offset":17280,"ancestor_record_offset":16368,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":492,"record_offset":16852,"parent_record_offset":16824,"ancestor_record_offset":16368,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":492,"record_offset":16824,"parent_record_offset":16796,"ancestor_record_offset":16368,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":492,"record_offset":16768,"parent_record_offset":16724,"ancestor_record_offset":16368,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":492,"record_offset":16796,"parent_record_offset":16768,"ancestor_record_offset":16368,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":492,"record_offset":16724,"parent_record_offset":16696,"ancestor_record_offset":16368,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":492,"record_offset":16696,"parent_record_offset":16668,"ancestor_record_offset":16368,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":492,"record_offset":16668,"parent_record_offset":16640,"ancestor_record_offset":16368,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$51::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":492,"record_offset":16560,"parent_record_offset":16536,"ancestor_record_offset":16368,"inlinee":17554,"source_line":2837,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":492,"record_offset":16888,"parent_record_offset":16852,"ancestor_record_offset":16368,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":492,"record_offset":16640,"parent_record_offset":16592,"ancestor_record_offset":16368,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":492,"record_offset":17168,"parent_record_offset":17144,"ancestor_record_offset":16368,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":492,"record_offset":17144,"parent_record_offset":16852,"ancestor_record_offset":16368,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":492,"record_offset":17232,"parent_record_offset":16592,"ancestor_record_offset":16368,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":492,"record_offset":17060,"parent_record_offset":17036,"ancestor_record_offset":16368,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":492,"record_offset":17488,"parent_record_offset":17436,"ancestor_record_offset":16368,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_shader_into_library<ref$<str$> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 210
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":492,"record_offset":17604,"rva":7022384,"va":5375731504}
// 编译实例 RVAs: 0x6b2730
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e16c0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6099213","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375731504,"raw_hex":"e8f3eea2ff","size":5,"soft_fail":false,"target_va":5369632448,"va":5375731656,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2981880","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375731504,"raw_hex":"e80880d2ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5375731571,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2982551","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375731504,"raw_hex":"e8697dd2ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5375732258,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140650460
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-402387","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375731504,"raw_hex":"e82ddcf9ff","size":5,"soft_fail":false,"target_va":5375329376,"va":5375731758,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406e8c80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t221726","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375731504,"raw_hex":"e81e620300","size":5,"soft_fail":false,"target_va":5375954048,"va":5375732317,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27861743","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375731504,"raw_hex":"e8ef22a901","size":5,"soft_fail":false,"target_va":5403593360,"va":5375731612,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27245942","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375731504,"raw_hex":"e876bd9f01","size":5,"soft_fail":false,"target_va":5402978144,"va":5375732197,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1794931","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375731504,"raw_hex":"e873631b00","size":5,"soft_fail":false,"target_va":5377527120,"va":5375732184,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1631966","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375731504,"raw_hex":"e8dee61800","size":5,"soft_fail":false,"target_va":5377364336,"va":5375732365,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27246696","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375731504,"raw_hex":"e868c09f01","size":5,"soft_fail":false,"target_va":5402978880,"va":5375732179,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27247958","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375731504,"raw_hex":"e856c59f01","size":5,"soft_fail":false,"target_va":5402980128,"va":5375732165,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::shader_library::AssetShaderLibrary::add_raster
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-42853","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375731504,"raw_hex":"e89b58ffff","size":5,"soft_fail":false,"target_va":5375689248,"va":5375732096,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":492,"record_offset":18728,"parent_record_offset":17604,"ancestor_record_offset":17604,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::shader::RasterShaderProgram::new_draw
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/shader.rs
//     evidence={"module":492,"record_offset":18600,"parent_record_offset":17604,"ancestor_record_offset":17604,"inlinee":26623,"source_line":94,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::pipeline_desc::RasterPipelineDesc::new_vertex_fragment
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":492,"record_offset":17756,"parent_record_offset":17604,"ancestor_record_offset":17604,"inlinee":27635,"source_line":24,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":492,"record_offset":18176,"parent_record_offset":18152,"ancestor_record_offset":17604,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":492,"record_offset":18200,"parent_record_offset":18176,"ancestor_record_offset":17604,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":492,"record_offset":18548,"parent_record_offset":18524,"ancestor_record_offset":17604,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":492,"record_offset":18152,"parent_record_offset":18108,"ancestor_record_offset":17604,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":492,"record_offset":18524,"parent_record_offset":18468,"ancestor_record_offset":17604,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":492,"record_offset":18496,"parent_record_offset":18468,"ancestor_record_offset":17604,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":492,"record_offset":18108,"parent_record_offset":18080,"ancestor_record_offset":17604,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":492,"record_offset":18080,"parent_record_offset":18052,"ancestor_record_offset":17604,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":492,"record_offset":18268,"parent_record_offset":18080,"ancestor_record_offset":17604,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":492,"record_offset":18468,"parent_record_offset":18444,"ancestor_record_offset":17604,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":492,"record_offset":18016,"parent_record_offset":17988,"ancestor_record_offset":17604,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":492,"record_offset":17988,"parent_record_offset":17960,"ancestor_record_offset":17604,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":492,"record_offset":17932,"parent_record_offset":17888,"ancestor_record_offset":17604,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":492,"record_offset":17960,"parent_record_offset":17932,"ancestor_record_offset":17604,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":492,"record_offset":17888,"parent_record_offset":17860,"ancestor_record_offset":17604,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":492,"record_offset":17860,"parent_record_offset":17832,"ancestor_record_offset":17604,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":492,"record_offset":17832,"parent_record_offset":17804,"ancestor_record_offset":17604,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":492,"record_offset":18052,"parent_record_offset":18016,"ancestor_record_offset":17604,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":492,"record_offset":17804,"parent_record_offset":17756,"ancestor_record_offset":17604,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":492,"record_offset":18332,"parent_record_offset":18308,"ancestor_record_offset":17604,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":492,"record_offset":18308,"parent_record_offset":18016,"ancestor_record_offset":17604,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":492,"record_offset":18396,"parent_record_offset":17756,"ancestor_record_offset":17604,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":492,"record_offset":18224,"parent_record_offset":18200,"ancestor_record_offset":17604,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":492,"record_offset":18652,"parent_record_offset":18600,"ancestor_record_offset":17604,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_mesh_into_library
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=100
// 源码行: 234
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x589E
// 首条位置: {"module":396,"inlinee":22686}
// 编译实例 RVAs: 0x61eec0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2377678","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375127232,"raw_hex":"e832b8dbff","size":5,"soft_fail":false,"target_va":5372749696,"va":5375127369,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406f12d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t860819","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375127232,"raw_hex":"e893220d00","size":5,"soft_fail":false,"target_va":5375988432,"va":5375127608,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28465946","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375127232,"raw_hex":"e81a5bb201","size":5,"soft_fail":false,"target_va":5403593360,"va":5375127409,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27850572","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375127232,"raw_hex":"e84cf7a801","size":5,"soft_fail":false,"target_va":5402978144,"va":5375127567,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2399563","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375127232,"raw_hex":"e84b9d2400","size":5,"soft_fail":false,"target_va":5377527120,"va":5375127552,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::mesh_library::AssetMeshLibrary::add
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-561459","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375127232,"raw_hex":"e8cd6ef7ff","size":5,"soft_fail":false,"target_va":5374566080,"va":5375127534,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::asset_libraries::Asset::name
//     owner=private_source_ownership_candidate; count=84
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":396,"record_offset":10204,"parent_record_offset":10140,"ancestor_record_offset":10140,"inlinee":22491,"source_line":35,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::Asset::new
//     owner=private_source_ownership_candidate; count=84
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":396,"record_offset":10176,"parent_record_offset":10140,"ancestor_record_offset":10140,"inlinee":21549,"source_line":31,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=84
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":396,"record_offset":10560,"parent_record_offset":10536,"ancestor_record_offset":10140,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=84
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":396,"record_offset":10584,"parent_record_offset":10560,"ancestor_record_offset":10140,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=84
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":396,"record_offset":10536,"parent_record_offset":10508,"ancestor_record_offset":10140,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=84
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":396,"record_offset":10508,"parent_record_offset":10480,"ancestor_record_offset":10140,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=84
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":396,"record_offset":10480,"parent_record_offset":10452,"ancestor_record_offset":10140,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=84
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":396,"record_offset":10656,"parent_record_offset":10480,"ancestor_record_offset":10140,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=84
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":396,"record_offset":10416,"parent_record_offset":10388,"ancestor_record_offset":10140,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=84
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":396,"record_offset":10388,"parent_record_offset":10360,"ancestor_record_offset":10140,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=84
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":396,"record_offset":10332,"parent_record_offset":10304,"ancestor_record_offset":10140,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=84
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":396,"record_offset":10360,"parent_record_offset":10332,"ancestor_record_offset":10140,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=84
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":396,"record_offset":10304,"parent_record_offset":10276,"ancestor_record_offset":10140,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=84
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":396,"record_offset":10276,"parent_record_offset":10248,"ancestor_record_offset":10140,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=84
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":396,"record_offset":10248,"parent_record_offset":10204,"ancestor_record_offset":10140,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=84
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":396,"record_offset":10452,"parent_record_offset":10416,"ancestor_record_offset":10140,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=84
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":396,"record_offset":10716,"parent_record_offset":10692,"ancestor_record_offset":10140,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=84
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":396,"record_offset":10692,"parent_record_offset":10416,"ancestor_record_offset":10140,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=84
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":396,"record_offset":10612,"parent_record_offset":10584,"ancestor_record_offset":10140,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00396.jsonl.gz"}
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
// 完整结构及原始类型记录: mod.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::startup::MinimalStartupPlugin
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 8
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::sealed::impl$9::add_to_app<bevy_app::plugin::sealed::PluginMarker,country_core::startup::MinimalStartupPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::gui::settings::SettingsMenuPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::resources::logo_animation::AnimatedLogo,bevy_app::plugin::sealed::PluginMarker,country_core::resources::loading_screen::LoadingScreen,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::vermeer::VermeerPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::mei::core::MeiPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::menu::SplashScreenPlugin>", "constructor": "bevy_app::plugin::sealed::impl$9::add_to_app", "arguments": ["bevy_app::plugin::sealed::PluginMarker", "country_core::startup::MinimalStartupPlugin", "bevy_app::plugin::sealed::PluginMarker", "tiny_glade::gui::settings::SettingsMenuPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::resources::logo_animation::AnimatedLogo", "bevy_app::plugin::sealed::PluginMarker", "country_core::resources::loading_screen::LoadingScreen", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::vermeer::VermeerPlugin", "bevy_app::plugin::sealed::PluginMarker", "toolbar::mei::core::MeiPlugin", "bevy_app::plugin::sealed::PluginMarker", "toolbar::menu::SplashScreenPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::startup::MinimalStartupPlugin>", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::startup::MinimalStartupPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any_mut<country_core::startup::MinimalStartupPlugin>", "constructor": "downcast_rs::impl$0::as_any_mut", "arguments": ["country_core::startup::MinimalStartupPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::into_any_rc<country_core::startup::MinimalStartupPlugin>", "constructor": "downcast_rs::impl$0::into_any_rc", "arguments": ["country_core::startup::MinimalStartupPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any<country_core::startup::MinimalStartupPlugin>", "constructor": "downcast_rs::impl$0::as_any", "arguments": ["country_core::startup::MinimalStartupPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::startup::MinimalStartupPlugin>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::startup::MinimalStartupPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::name<country_core::startup::MinimalStartupPlugin>", "constructor": "bevy_app::plugin::Plugin::name", "arguments": ["country_core::startup::MinimalStartupPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::finish<country_core::startup::MinimalStartupPlugin>", "constructor": "bevy_app::plugin::Plugin::finish", "arguments": ["country_core::startup::MinimalStartupPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: mod.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: mod.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 206; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::system::commands::Commands", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::game_loading_audio::GameLoadingAudioState>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>", "bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>", "enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >", "bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>", "bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::game_loading_audio::GameLoadingAudioState>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::game_loading_audio::GameLoadingAudioState>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<ref_mut$<country_core::resources::prefab::PrefabInstance>,tuple$<> >,bevy_ecs::event::EventReader<country_core::systems::prefab::DespawnPrefabCmd>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::system::query::Query<ref_mut$<country_core::resources::prefab::PrefabInstance>,tuple$<> >", "bevy_ecs::event::EventReader<country_core::systems::prefab::DespawnPrefabCmd>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>", "bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref_mut$<country_core::resources::prefab::PrefabInstance>,tuple$<> >
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::prefab::DespawnPrefabCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::mouse_capture::CursorCapturedPosition>,bevy_ecs::change_detection::ResMut<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::event::EventWriter<country_core::resources::window::CursorMoved>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_capture::CursorCapturedPosition>", "bevy_ecs::change_detection::ResMut<country_core::resources::mouse_terrain_raycast::CursorPositionSS>", "bevy_ecs::event::EventWriter<country_core::resources::window::CursorMoved>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_capture::CursorCapturedPosition>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::window::CursorMoved>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::event::EventReader<country_core::systems::audio::building_audio::PlayBuildingAudio>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::building_audio::BuildingAudioState>,bevy_ecs::event::EventWriter<country_core::resources::events::CmdSuppressMusic>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::event::EventReader<country_core::systems::audio::building_audio::PlayBuildingAudio>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::building_audio::BuildingAudioState>", "bevy_ecs::event::EventWriter<country_core::resources::events::CmdSuppressMusic>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::audio::building_audio::PlayBuildingAudio>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::building_audio::BuildingAudioState>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::events::CmdSuppressMusic>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::startup::impl$0::add_game_startup_systems
// 信息 ID: c2c9fc88c13977fa6f057e8c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x3c5ebe -> 0x67b3d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::impl$0::add_game_startup_systems::closure$0
// 信息 ID: b2db91988d849cefc44aade6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 2d6fe8a5858bc1c30f3105fe: void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::startup::load_glade_settings::GladeSettings>)
// [泛型宿主/闭包/直接引用方参数候选] 4e1aad1437a3aaa713247c09: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::startup::load_glade_settings::GladeSettings>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)
// [泛型宿主/闭包/直接引用方参数候选] 4e9a32f08ed9cdbad86c0d7e: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>)
// [泛型宿主/闭包/直接引用方参数候选] 5b63364bde1be456ca8b15f8: void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::color_library::ColorLibrarySsbo>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)
// [泛型宿主/闭包/直接引用方参数候选] 5cfc7ffa8158e7c77ea39dca: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TodSequence>,bevy_ecs::change_detection::ResMut<country_core::resources::weather::Weather>,bevy_ecs::change_detection::Res<country_core::startup::load_glade_settings::GladeSettings>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)
// [泛型宿主/闭包/直接引用方参数候选] 5ead4c28cdae23fe8b291595: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)
// [泛型宿主/闭包/直接引用方参数候选] 5f057b0cd510a3a61c129b07: void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::music_audio::MusicAudioState>)
// [泛型宿主/闭包/直接引用方参数候选] 6cf9bc64ac60be34bc4af2f7: void (*)(bevy_ecs::change_detection::Res<country_core::startup::load_glade_settings::GladeSettings>,bevy_ecs::change_detection::ResMut<country_core::startup::load_glade_default_customization::GladeDefaultCustomization>)
// 其余候选接口见完整附档，共 17 条。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::startup::impl$0::add_game_startup_systems", "captures": "unknown"}
// [闭包环境类型引用] country_core::startup::impl$0::add_game_startup_systems::closure_env$0 <- bevy_ecs::system::function_system::impl$7::initialize<void (*)(bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),country_core::startup::impl$0::add_game_startup_systems::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::startup::impl$0::add_game_startup_systems::closure_env$0 <- bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),country_core::startup::impl$0::add_game_startup_systems::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::startup::impl$0::add_game_startup_systems::closure_env$0 <- bevy_ecs::system::function_system::impl$7::has_deferred<void (*)(bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),country_core::startup::impl$0::add_game_startup_systems::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::startup::impl$0::add_game_startup_systems::closure_env$0 <- bevy_ecs::system::function_system::impl$7::set_last_run<void (*)(bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),country_core::startup::impl$0::add_game_startup_systems::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::startup::impl$0::add_game_startup_systems::closure_env$0 <- bevy_ecs::system::function_system::impl$7::apply_deferred<void (*)(bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),country_core::startup::impl$0::add_game_startup_systems::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::startup::impl$0::add_game_startup_systems::closure_env$0 <- bevy_ecs::system::function_system::impl$7::queue_deferred<void (*)(bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),country_core::startup::impl$0::add_game_startup_systems::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::startup::impl$0::add_game_startup_systems::closure_env$0 <- bevy_ecs::system::function_system::impl$7::check_change_tick<void (*)(bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),country_core::startup::impl$0::add_game_startup_systems::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::startup::impl$0::add_game_startup_systems::closure_env$0 <- bevy_ecs::system::function_system::impl$7::archetype_component_access<void (*)(bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),country_core::startup::impl$0::add_game_startup_systems::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 25 条。
// [PDB 内联宿主] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),country_core::startup::impl$0::add_game_startup_systems::closure_env$0>; module=492, inline_record=4660; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),country_core::startup::impl$0::add_game_startup_systems::closure_env$0> >; module=1325, inline_record=2257484; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::impl$0::add_game_startup_systems::closure$1
// 信息 ID: 074b9f3c593b68fd9f659cab
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 2d3474f22838f8c14b857237: void (*)(bevy_ecs::event::EventWriter<country_core::resources::events::ClearCanvasCmd>)
// [泛型宿主/闭包/直接引用方参数候选] cc85d15c013966493a85b83e: void (*)(bevy_ecs::system::query::Query<bevy_ecs::entity::Entity,bevy_ecs::query::filter::Without<country_core::components::KeepOnLevelUnload> >,bevy_ecs::system::commands::Commands)
// [泛型宿主/闭包/直接引用方参数候选] f1571263866702a2d9c5663c: void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::game_loading_audio::GameLoadingAudioState>)
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::startup::impl$0::add_game_startup_systems", "captures": "unknown"}
// [闭包环境类型引用] country_core::startup::impl$0::add_game_startup_systems::closure_env$1 <- bevy_ecs::system::function_system::impl$7::initialize<void (*)(bevy_ecs::event::EventWriter<country_core::resources::events::ClearCanvasCmd>),country_core::startup::impl$0::add_game_startup_systems::closure_env$1>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::startup::impl$0::add_game_startup_systems::closure_env$1 <- bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::event::EventWriter<country_core::resources::events::ClearCanvasCmd>),country_core::startup::impl$0::add_game_startup_systems::closure_env$1>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::startup::impl$0::add_game_startup_systems::closure_env$1 <- bevy_ecs::system::function_system::impl$7::get_last_run<void (*)(bevy_ecs::event::EventWriter<country_core::resources::events::ClearCanvasCmd>),country_core::startup::impl$0::add_game_startup_systems::closure_env$1>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::startup::impl$0::add_game_startup_systems::closure_env$1 <- bevy_ecs::system::function_system::impl$7::is_exclusive<void (*)(bevy_ecs::event::EventWriter<country_core::resources::events::ClearCanvasCmd>),country_core::startup::impl$0::add_game_startup_systems::closure_env$1>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::startup::impl$0::add_game_startup_systems::closure_env$1 <- bevy_ecs::system::function_system::impl$7::component_access<void (*)(bevy_ecs::event::EventWriter<country_core::resources::events::ClearCanvasCmd>),country_core::startup::impl$0::add_game_startup_systems::closure_env$1>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::startup::impl$0::add_game_startup_systems::closure_env$1 <- bevy_ecs::system::system::System::type_id<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::event::EventWriter<country_core::resources::events::ClearCanvasCmd>),country_core::startup::impl$0::add_game_startup_systems::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::startup::impl$0::add_game_startup_systems::closure_env$1 <- bevy_ecs::schedule::config::IntoSystemConfigs::chain<tuple$<country_core::startup::impl$0::add_game_startup_systems::closure_env$1,tuple$<void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::game_loading_audio::GameLoadingAudioState>),void (*)(bevy_ecs::system::query::Query<bevy_ecs::entity::Entity,bevy_ecs::query::filter::Without<country_core::components::KeepOnLevelUnload> >,bevy_ecs::system::commands::Commands)> >,tuple$<bevy_ecs::schedule::config::SystemConfigTupleMarker,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::event::EventWriter<country_core::resources::events::ClearCanvasCmd>)>,tuple$<bevy_ecs::schedule::config::SystemConfigTupleMarker,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::game_loading_audio::GameLoadingAudioState>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<bevy_ecs::entity::Entity,bevy_ecs::query::filter::Without<country_core::components::KeepOnLevelUnload> >,bevy_ecs::system::commands::Commands)> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::startup::impl$0::add_game_startup_systems::closure_env$1 <- bevy_ecs::schedule::set::impl$7::system_type<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::event::EventWriter<country_core::resources::events::ClearCanvasCmd>),country_core::startup::impl$0::add_game_startup_systems::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 20 条。
// [PDB 内联宿主] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::event::EventWriter<country_core::resources::events::ClearCanvasCmd>),country_core::startup::impl$0::add_game_startup_systems::closure_env$1>; module=492, inline_record=2844; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::event::EventWriter<country_core::resources::events::ClearCanvasCmd>),country_core::startup::impl$0::add_game_startup_systems::closure_env$1> >; module=1325, inline_record=1435948; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::impl$1::build
// 信息 ID: 64b64b9418434484e7e9c2b8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_mesh_into_library
// 信息 ID: c4baef2890a38b29c2986015
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 83 条；闭包环境类型引用 0 条。
// [静态直接引用] system_clutter::clutter::load_clutter_meshes::load_clutter_meshes::closure$0 @ 0x1ac9e1a -> 0x61eec0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::try_trait::impl$0::wrap_mut_2::closure$0<tuple$<>,tuple$<>,i32,core::iter::adapters::map::map_fold::closure_env$0<i32,ref$<system_clutter::clutter::load_clutter_meshes::ClutterMesh>,tuple$<>,system_clutter::clutter::load_clutter_meshes::load_clutter_meshes::closure_env$1,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<ref$<system_clutter::clutter::load_clutter_meshes::ClutterMesh>,alloc::vec::impl$19::extend_trusted::closure_env$0<ref$<system_clutter::clutter::load_clutter_meshes::ClutterMesh>,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::RangeInclusive<i32>,system_clutter::clutter::load_clutter_meshes::load_clutter_meshes::closure_env$1> > > > > @ 0x1ae0eca -> 0x61eec0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::load_decorators::load_decorators @ 0x1c43c39 -> 0x61eec0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::startup::load_glade_tod_and_weather::load_snow_particles; module=396, inline_record=10140; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::load_glade_tod_and_weather::load_rain_particles; module=396, inline_record=11372; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::spawn_human_ref::spawn_human_ref; module=396, inline_record=12316; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::startup_garden_brush::startup_garden_brush; module=396, inline_record=13248; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::startup_garden_brush::startup_garden_brush; module=396, inline_record=13988; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::startup_grass::startup_grass; module=398, inline_record=51148; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::startup_sheep::startup_sheep; module=419, inline_record=32032; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::startup_sheep::startup_sheep; module=419, inline_record=33488; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_shader_into_library<alloc::string::String>
// 信息 ID: a810f470509e6e5e4a298564
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::startup::load_shader_into_library -> ["alloc::string::String"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::clearing::init_clearing @ 0x6c9370 -> 0x6b23a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_shader_into_library<ref$<str$> >
// 信息 ID: 743964dcdb5306bab74a53af
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 0a0bc4d76faec5be35e32390: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::draw_passes::deferred_raster::DeferredRasterResources>)
// [泛型宿主/闭包/直接引用方参数候选] cd0996daac918459f879cc27: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::draw_passes::blit::BlitResources>)
// [泛型宿主/闭包/直接引用方参数候选] f6e723bf3a293de264db1c5c: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>)
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] country_core::startup::load_shader_into_library -> ["ref$<str$>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 99 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::load_glade_tod_and_weather::load_snow_particles @ 0x538410 -> 0x6b2730 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_glade_tod_and_weather::load_rain_particles @ 0x5386e0 -> 0x6b2730 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_garden_brush::startup_garden_brush @ 0x538b9e -> 0x6b2730 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_garden_brush::startup_garden_brush @ 0x538d52 -> 0x6b2730 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_grass::startup_grass @ 0x5428bc -> 0x6b2730 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_grass::startup_grass @ 0x542a75 -> 0x6b2730 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_grass::startup_grass @ 0x542af9 -> 0x6b2730 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>)> @ 0x57bbf1 -> 0x6b2730 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>)> @ 0x57bc24 -> 0x6b2730 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>)> @ 0x57bfa4 -> 0x6b2730 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>)> @ 0x57bfd3 -> 0x6b2730 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>)> @ 0x57c002 -> 0x6b2730 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>)> @ 0x57c031 -> 0x6b2730 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_sheep::startup_sheep @ 0x598705 -> 0x6b2730 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::logo_animation::startup_logo_test @ 0x59ad08 -> 0x6b2730 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::logo_animation::startup_logo_test @ 0x59aebc -> 0x6b2730 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
