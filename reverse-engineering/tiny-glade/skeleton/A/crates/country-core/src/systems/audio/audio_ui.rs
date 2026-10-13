// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/audio/audio_ui.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: e3aabc36cebfe5e7b8204313f622c0d915aaa599e3d05bf80e307ff7ac58c5e4
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: audio_ui.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 1359
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
// use egui; // namespace candidate
//   package identities: [{"name":"egui","source":"PDB_registry_path_only","version":"0.27.2"}]
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"PDB_registry_path_only","version":"0.4.9"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"PDB_registry_path_only","version":"0.12.1"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::audio::audio_ui::audio_ui
// 记录形式: compiled_procedure; instances=1
// 源码行: 7
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1359,"record_offset":37088,"rva":15792768,"va":5384501888}
// 编译实例 RVAs: 0xf0fa80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140024d80
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: audio_ui.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15642172","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384501888,"raw_hex":"e8c45111ff","size":5,"soft_fail":false,"target_va":5368860032,"va":5384502199,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14019f810
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: audio_ui.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14090948","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384501888,"raw_hex":"e83cfd28ff","size":5,"soft_fail":false,"target_va":5370411024,"va":5384501967,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: audio_ui.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11752343","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384501888,"raw_hex":"e869ac4cff","size":5,"soft_fail":false,"target_va":5372749696,"va":5384502034,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ea0c90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: audio_ui.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-454365","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384501888,"raw_hex":"e82311f9ff","size":5,"soft_fail":false,"target_va":5384047760,"va":5384502120,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14101b210
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: audio_ui.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1095365","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384501888,"raw_hex":"e8c5b61000","size":5,"soft_fail":false,"target_va":5385597456,"va":5384502086,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: audio_ui.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19091343","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384501888,"raw_hex":"e88f4f2301","size":5,"soft_fail":false,"target_va":5403593360,"va":5384502012,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18475989","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384501888,"raw_hex":"e8d5eb1901","size":5,"soft_fail":false,"target_va":5402978144,"va":5384502150,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":1359,"record_offset":37320,"parent_record_offset":37088,"ancestor_record_offset":37088,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$31::as_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":1359,"record_offset":37236,"parent_record_offset":37088,"ancestor_record_offset":37088,"inlinee":23077,"source_line":334,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1359,"record_offset":37652,"parent_record_offset":37088,"ancestor_record_offset":37088,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_direct] egui::containers::window::Window::open
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/egui-0.27.2/src/containers/window.rs
//     evidence={"module":1359,"record_offset":37376,"parent_record_offset":37088,"ancestor_record_offset":37088,"inlinee":11105,"source_line":81,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_direct] egui::containers::window::Window::show
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/egui-0.27.2/src/containers/window.rs
//     evidence={"module":1359,"record_offset":37404,"parent_record_offset":37088,"ancestor_record_offset":37088,"inlinee":11106,"source_line":365,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1359,"record_offset":37520,"parent_record_offset":37496,"ancestor_record_offset":37088,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1359,"record_offset":37544,"parent_record_offset":37520,"ancestor_record_offset":37088,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1359,"record_offset":37460,"parent_record_offset":37432,"ancestor_record_offset":37088,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1359,"record_offset":37496,"parent_record_offset":37460,"ancestor_record_offset":37088,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":1359,"record_offset":37432,"parent_record_offset":37404,"ancestor_record_offset":37088,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1359,"record_offset":37816,"parent_record_offset":37780,"ancestor_record_offset":37088,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1359,"record_offset":37780,"parent_record_offset":37752,"ancestor_record_offset":37088,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":1359,"record_offset":37284,"parent_record_offset":37260,"ancestor_record_offset":37088,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":1359,"record_offset":37260,"parent_record_offset":37236,"ancestor_record_offset":37088,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1359,"record_offset":37620,"parent_record_offset":37404,"ancestor_record_offset":37088,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1359,"record_offset":37848,"parent_record_offset":37816,"ancestor_record_offset":37088,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1359,"record_offset":37572,"parent_record_offset":37544,"ancestor_record_offset":37088,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1359,"record_offset":37876,"parent_record_offset":37816,"ancestor_record_offset":37088,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1359,"record_offset":37908,"parent_record_offset":37780,"ancestor_record_offset":37088,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::audio::audio_ui::audio_ui::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF730
// 首条位置: {"module":1359,"inlinee":63280}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1359,"record_offset":27208,"parent_record_offset":27052,"ancestor_record_offset":27052,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1359,"record_offset":27456,"parent_record_offset":27052,"ancestor_record_offset":27052,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_direct] core::time::Duration::as_secs_f32
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\time.rs
//     evidence={"module":1359,"record_offset":29616,"parent_record_offset":27052,"ancestor_record_offset":27052,"inlinee":12632,"source_line":855,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::audio::music_audio::MusicAudioState::current_track
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/audio/music_audio.rs
//     evidence={"module":1359,"record_offset":28824,"parent_record_offset":27052,"ancestor_record_offset":27052,"inlinee":62002,"source_line":66,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::audio::music_audio::MusicAudioState::music_intensity
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/audio/music_audio.rs
//     evidence={"module":1359,"record_offset":27180,"parent_record_offset":27052,"ancestor_record_offset":27052,"inlinee":62001,"source_line":62,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::audio::music_audio::MusicAudioState::next_track
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/audio/music_audio.rs
//     evidence={"module":1359,"record_offset":33768,"parent_record_offset":27052,"ancestor_record_offset":27052,"inlinee":62005,"source_line":96,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::audio::music_audio::MusicAudioState::track_elapsed
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/audio/music_audio.rs
//     evidence={"module":1359,"record_offset":29588,"parent_record_offset":27052,"ancestor_record_offset":27052,"inlinee":62003,"source_line":70,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_direct] egui::response::Response::clicked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/egui-0.27.2/src/response.rs
//     evidence={"module":1359,"record_offset":32648,"parent_record_offset":27052,"ancestor_record_offset":27052,"inlinee":5782,"source_line":137,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_direct] egui::ui::Ui::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/egui-0.27.2/src/ui.rs
//     evidence={"module":1359,"record_offset":32340,"parent_record_offset":27052,"ancestor_record_offset":27052,"inlinee":6449,"source_line":1116,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_direct] egui::ui::Ui::button
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/egui-0.27.2/src/ui.rs
//     evidence={"module":1359,"record_offset":32620,"parent_record_offset":27052,"ancestor_record_offset":27052,"inlinee":6450,"source_line":1450,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_direct] egui::ui::Ui::label
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"egui","version":"0.27.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/egui-0.27.2/src/ui.rs
//     evidence={"module":1359,"record_offset":27304,"parent_record_offset":27052,"ancestor_record_offset":27052,"inlinee":5804,"source_line":1298,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_direct] egui::ui::Ui::separator
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"egui","version":"0.27.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/egui-0.27.2/src/ui.rs
//     evidence={"module":1359,"record_offset":27708,"parent_record_offset":27052,"ancestor_record_offset":27052,"inlinee":6536,"source_line":1559,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_direct] egui::widgets::slider::Slider::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/egui-0.27.2/src/widgets/slider.rs
//     evidence={"module":1359,"record_offset":30144,"parent_record_offset":27052,"ancestor_record_offset":27052,"inlinee":6446,"source_line":95,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_direct] egui::widgets::slider::Slider::text
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/egui-0.27.2/src/widgets/slider.rs
//     evidence={"module":1359,"record_offset":30440,"parent_record_offset":27052,"ancestor_record_offset":27052,"inlinee":6448,"source_line":165,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1359,"record_offset":30332,"parent_record_offset":30308,"ancestor_record_offset":27052,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1359,"record_offset":30356,"parent_record_offset":30332,"ancestor_record_offset":27052,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1359,"record_offset":31356,"parent_record_offset":31332,"ancestor_record_offset":27052,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1359,"record_offset":30276,"parent_record_offset":30244,"ancestor_record_offset":27052,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1359,"record_offset":30308,"parent_record_offset":30276,"ancestor_record_offset":27052,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1359,"record_offset":31332,"parent_record_offset":31276,"ancestor_record_offset":27052,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":1359,"record_offset":30244,"parent_record_offset":30188,"ancestor_record_offset":27052,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::buffer_read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1359,"record_offset":33988,"parent_record_offset":33812,"ancestor_record_offset":27052,"inlinee":10717,"source_line":165,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::front
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1359,"record_offset":28852,"parent_record_offset":28824,"ancestor_record_offset":27052,"inlinee":22347,"source_line":1520,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1359,"record_offset":28876,"parent_record_offset":28852,"ancestor_record_offset":27052,"inlinee":7687,"source_line":667,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1359,"record_offset":33852,"parent_record_offset":33812,"ancestor_record_offset":27052,"inlinee":11193,"source_line":1254,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::pop_front
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1359,"record_offset":33812,"parent_record_offset":33768,"ancestor_record_offset":27052,"inlinee":10716,"source_line":1610,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1359,"record_offset":28908,"parent_record_offset":28876,"ancestor_record_offset":27052,"inlinee":7712,"source_line":159,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1359,"record_offset":28964,"parent_record_offset":28876,"ancestor_record_offset":27052,"inlinee":7688,"source_line":200,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1359,"record_offset":28988,"parent_record_offset":28964,"ancestor_record_offset":27052,"inlinee":7689,"source_line":195,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::impl$21::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1359,"record_offset":34128,"parent_record_offset":34104,"ancestor_record_offset":27052,"inlinee":23774,"source_line":2875,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::wrap_index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1359,"record_offset":29012,"parent_record_offset":28988,"ancestor_record_offset":27052,"inlinee":11291,"source_line":2679,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1359,"record_offset":27264,"parent_record_offset":27236,"ancestor_record_offset":27052,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1359,"record_offset":31304,"parent_record_offset":31276,"ancestor_record_offset":27052,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1359,"record_offset":28932,"parent_record_offset":28908,"ancestor_record_offset":27052,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1359,"record_offset":30864,"parent_record_offset":30836,"ancestor_record_offset":27052,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1359,"record_offset":30836,"parent_record_offset":30808,"ancestor_record_offset":27052,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1359,"record_offset":31012,"parent_record_offset":30836,"ancestor_record_offset":27052,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1359,"record_offset":31276,"parent_record_offset":31248,"ancestor_record_offset":27052,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1359,"record_offset":30772,"parent_record_offset":30744,"ancestor_record_offset":27052,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1359,"record_offset":30744,"parent_record_offset":30716,"ancestor_record_offset":27052,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1359,"record_offset":30688,"parent_record_offset":30660,"ancestor_record_offset":27052,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1359,"record_offset":30716,"parent_record_offset":30688,"ancestor_record_offset":27052,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1359,"record_offset":30660,"parent_record_offset":30632,"ancestor_record_offset":27052,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":1359,"record_offset":30632,"parent_record_offset":30604,"ancestor_record_offset":27052,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1359,"record_offset":30604,"parent_record_offset":30576,"ancestor_record_offset":27052,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1359,"record_offset":27572,"parent_record_offset":27536,"ancestor_record_offset":27052,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1359,"record_offset":32772,"parent_record_offset":32740,"ancestor_record_offset":27052,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1359,"record_offset":27536,"parent_record_offset":27508,"ancestor_record_offset":27052,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::into_raw_parts_with_alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1359,"record_offset":34152,"parent_record_offset":34128,"ancestor_record_offset":27052,"inlinee":23722,"source_line":900,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1359,"record_offset":30808,"parent_record_offset":30772,"ancestor_record_offset":27052,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1359,"record_offset":33056,"parent_record_offset":33032,"ancestor_record_offset":27052,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":1359,"record_offset":27364,"parent_record_offset":27332,"ancestor_record_offset":27052,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1359,"record_offset":31072,"parent_record_offset":31048,"ancestor_record_offset":27052,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\manually_drop.rs
//     evidence={"module":1359,"record_offset":34176,"parent_record_offset":34152,"ancestor_record_offset":27052,"inlinee":4174,"source_line":70,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1359,"record_offset":34332,"parent_record_offset":34300,"ancestor_record_offset":27052,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1359,"record_offset":33936,"parent_record_offset":33908,"ancestor_record_offset":27052,"inlinee":7690,"source_line":1639,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1359,"record_offset":27236,"parent_record_offset":27208,"ancestor_record_offset":27052,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1359,"record_offset":29056,"parent_record_offset":28824,"ancestor_record_offset":27052,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1359,"record_offset":31048,"parent_record_offset":30772,"ancestor_record_offset":27052,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=40
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1359,"record_offset":27480,"parent_record_offset":27456,"ancestor_record_offset":27052,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1359,"record_offset":27604,"parent_record_offset":27572,"ancestor_record_offset":27052,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1359,"record_offset":30384,"parent_record_offset":30356,"ancestor_record_offset":27052,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1359,"record_offset":32968,"parent_record_offset":32884,"ancestor_record_offset":27052,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1359,"record_offset":32992,"parent_record_offset":32968,"ancestor_record_offset":27052,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1359,"record_offset":33228,"parent_record_offset":33196,"ancestor_record_offset":27052,"inlinee":5796,"source_line":2580,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1359,"record_offset":32912,"parent_record_offset":32884,"ancestor_record_offset":27052,"inlinee":5726,"source_line":2629,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1359,"record_offset":33252,"parent_record_offset":33228,"ancestor_record_offset":27052,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1359,"record_offset":32936,"parent_record_offset":32912,"ancestor_record_offset":27052,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1359,"record_offset":27632,"parent_record_offset":27572,"ancestor_record_offset":27052,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1359,"record_offset":27664,"parent_record_offset":27536,"ancestor_record_offset":27052,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] egui::context::Context::input
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/egui-0.27.2/src/context.rs
//     evidence={"module":1359,"record_offset":32712,"parent_record_offset":32684,"ancestor_record_offset":27052,"inlinee":5798,"source_line":795,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] egui::context::Context::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/egui-0.27.2/src/context.rs
//     evidence={"module":1359,"record_offset":32740,"parent_record_offset":32712,"ancestor_record_offset":27052,"inlinee":5785,"source_line":717,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] egui::context::impl$13::input::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/egui-0.27.2/src/context.rs
//     evidence={"module":1359,"record_offset":33092,"parent_record_offset":32740,"ancestor_record_offset":27052,"inlinee":5800,"source_line":796,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] egui::response::Response::clicked_by
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/egui-0.27.2/src/response.rs
//     evidence={"module":1359,"record_offset":32684,"parent_record_offset":32648,"ancestor_record_offset":27052,"inlinee":5797,"source_line":150,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] egui::response::impl$0::clicked_by::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/egui-0.27.2/src/response.rs
//     evidence={"module":1359,"record_offset":33116,"parent_record_offset":33092,"ancestor_record_offset":27052,"inlinee":5802,"source_line":151,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] egui::widget_text::RichText::new
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"egui","version":"0.27.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/egui-0.27.2/src/widget_text.rs
//     evidence={"module":1359,"record_offset":27412,"parent_record_offset":27388,"ancestor_record_offset":27052,"inlinee":5819,"source_line":79,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] egui::widget_text::impl$10::from
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"egui","version":"0.27.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/egui-0.27.2/src/widget_text.rs
//     evidence={"module":1359,"record_offset":27388,"parent_record_offset":27364,"ancestor_record_offset":27052,"inlinee":6503,"source_line":707,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] egui::widget_text::impl$8::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/egui-0.27.2/src/widget_text.rs
//     evidence={"module":1359,"record_offset":30520,"parent_record_offset":30492,"ancestor_record_offset":27052,"inlinee":5817,"source_line":693,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] egui::widgets::label::Label::new
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"egui","version":"0.27.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/egui-0.27.2/src/widgets/label.rs
//     evidence={"module":1359,"record_offset":27332,"parent_record_offset":27304,"ancestor_record_offset":27052,"inlinee":5806,"source_line":32,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] egui::widgets::separator::impl$0::default
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"egui","version":"0.27.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/egui-0.27.2/src/widgets/separator.rs
//     evidence={"module":1359,"record_offset":27732,"parent_record_offset":27708,"ancestor_record_offset":27052,"inlinee":6538,"source_line":22,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] egui::widgets::slider::Slider::from_get_set
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/egui-0.27.2/src/widgets/slider.rs
//     evidence={"module":1359,"record_offset":30188,"parent_record_offset":30144,"ancestor_record_offset":27052,"inlinee":6447,"source_line":111,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::RwLock::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1359,"record_offset":32860,"parent_record_offset":32772,"ancestor_record_offset":27052,"inlinee":5786,"source_line":479,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::RwLockWriteGuard::map
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1359,"record_offset":33032,"parent_record_offset":32772,"ancestor_record_offset":27052,"inlinee":5789,"source_line":1494,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$52::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1359,"record_offset":33172,"parent_record_offset":33148,"ancestor_record_offset":27052,"inlinee":5794,"source_line":2576,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_exclusive
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1359,"record_offset":32884,"parent_record_offset":32860,"ancestor_record_offset":27052,"inlinee":5787,"source_line":67,"archive":"_pdb/A/modules/01359.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_exclusive
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1359,"record_offset":33196,"parent_record_offset":33172,"ancestor_record_offset":27052,"inlinee":5795,"source_line":94,"archive":"_pdb/A/modules/01359.jsonl.gz"}
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
// 完整结构及原始类型记录: audio_ui.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: audio_ui.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: audio_ui.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 8; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::Res<country_core::input::input_stream::InputRecordReplayState>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>", "bevy_ecs::change_detection::Res<country_core::resources::history::History>", "bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>", "bevy_ecs::change_detection::Res<country_core::input::input_stream::InputRecordReplayState>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::history::History>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::input_stream::InputRecordReplayState>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TodSequence>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TodSequence>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TodSequence>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::draw_passes::blit::BlitResources>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::draw_passes::blit::BlitResources>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::draw_passes::blit::BlitResources>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::draw_passes::deferred_raster::DeferredRasterResources>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::draw_passes::deferred_raster::DeferredRasterResources>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::draw_passes::deferred_raster::DeferredRasterResources>
// [文件级接口候选，具体函数归属 unknown] void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >,bevy_ecs::change_detection::ResMut<country_core::systems::audio::music_audio::MusicAudioState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>)
// 候选参数类型: ["enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::music_audio::MusicAudioState>", "bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::music_audio::MusicAudioState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>

// ------------------------------------------------------------------
// 函数: country_core::systems::audio::audio_ui::audio_ui
// 信息 ID: a709a8ef84f66da59f6ba5a6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 69b1da42d665b7cf307662e7: void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >,bevy_ecs::change_detection::ResMut<country_core::systems::audio::music_audio::MusicAudioState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >,bevy_ecs::change_detection::ResMut<country_core::systems::audio::music_audio::MusicAudioState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>),void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >,bevy_ecs::change_detection::ResMut<country_core::systems::audio::music_audio::MusicAudioState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>)> @ 0x5c6d3f -> 0xf0fa80 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >,bevy_ecs::change_detection::ResMut<country_core::systems::audio::music_audio::MusicAudioState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>),void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >,bevy_ecs::change_detection::ResMut<country_core::systems::audio::music_audio::MusicAudioState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>)> > @ 0xe4c7da -> 0xf0fa80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::audio::audio_ui::audio_ui::closure$0
// 信息 ID: 46f6379ba3c4baf66db566c2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::audio::audio_ui::audio_ui", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::audio::audio_ui::audio_ui::closure_env$0 <- core::ops::function::FnOnce::call_once<country_core::systems::audio::audio_ui::audio_ui::closure_env$0,tuple$<ref_mut$<egui::ui::Ui> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::audio::audio_ui::audio_ui::closure_env$0 <- core::ptr::drop_in_place<country_core::systems::audio::audio_ui::audio_ui::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::audio::audio_ui::audio_ui::closure_env$0,tuple$<ref_mut$<egui::ui::Ui> > >; module=1359, inline_record=27052; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
