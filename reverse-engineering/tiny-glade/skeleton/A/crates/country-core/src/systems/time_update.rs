// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/time_update.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 08815a44575f9259633f3437bde690df20b10fbb0f321016da4c97471b4585c0
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: time_update.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 1328
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
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::time_update::time_update
// 记录形式: compiled_procedure; instances=1
// 源码行: 10
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1328,"record_offset":30852,"rva":15277424,"va":5383986544}
// 编译实例 RVAs: 0xe91d70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1420de270
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: time_update.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19186093","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383986544,"raw_hex":"e8adc12401","size":5,"soft_fail":false,"target_va":5403173488,"va":5383987390,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t18991556","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383986544,"raw_hex":"e8c4c92101","size":5,"soft_fail":false,"target_va":5402978912,"va":5383987351,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":1328,"record_offset":31424,"parent_record_offset":30852,"ancestor_record_offset":30852,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1328,"record_offset":32240,"parent_record_offset":30852,"ancestor_record_offset":30852,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_direct] core::time::Duration::from_secs_f32
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\time.rs
//     evidence={"module":1328,"record_offset":31552,"parent_record_offset":30852,"ancestor_record_offset":30852,"inlinee":12633,"source_line":968,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_direct] core::time::impl$3::add_assign
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\time.rs
//     evidence={"module":1328,"record_offset":31808,"parent_record_offset":30852,"ancestor_record_offset":30852,"inlinee":31330,"source_line":1115,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::gui::EscMenuState::is_closed
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/gui.rs
//     evidence={"module":1328,"record_offset":31772,"parent_record_offset":30852,"ancestor_record_offset":30852,"inlinee":23510,"source_line":143,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::shader_refactoring_state::ShaderRefactoringState::disabled
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/shader_refactoring_state.rs
//     evidence={"module":1328,"record_offset":31008,"parent_record_offset":30852,"ancestor_record_offset":30852,"inlinee":26429,"source_line":36,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::time_update::calculate_delta_seconds
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/time_update.rs
//     evidence={"module":1328,"record_offset":31032,"parent_record_offset":30852,"ancestor_record_offset":30852,"inlinee":61991,"source_line":42,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1328,"record_offset":31144,"parent_record_offset":31116,"ancestor_record_offset":30852,"inlinee":7687,"source_line":667,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1328,"record_offset":31292,"parent_record_offset":31144,"ancestor_record_offset":30852,"inlinee":7712,"source_line":159,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1328,"record_offset":31176,"parent_record_offset":31144,"ancestor_record_offset":30852,"inlinee":7688,"source_line":200,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1328,"record_offset":31208,"parent_record_offset":31176,"ancestor_record_offset":30852,"inlinee":7689,"source_line":195,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::impl$12::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1328,"record_offset":31116,"parent_record_offset":31088,"ancestor_record_offset":30852,"inlinee":21861,"source_line":2771,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1328,"record_offset":31316,"parent_record_offset":31292,"ancestor_record_offset":30852,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":1328,"record_offset":31452,"parent_record_offset":31424,"ancestor_record_offset":30852,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::impl$46::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\num.rs
//     evidence={"module":1328,"record_offset":31708,"parent_record_offset":31592,"ancestor_record_offset":30852,"inlinee":6006,"source_line":76,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::to_bits
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1328,"record_offset":31652,"parent_record_offset":31592,"ancestor_record_offset":30852,"inlinee":12635,"source_line":1110,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::to_bits::rt_f32_to_u32
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1328,"record_offset":31676,"parent_record_offset":31652,"ancestor_record_offset":30852,"inlinee":12637,"source_line":1150,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1328,"record_offset":31240,"parent_record_offset":31208,"ancestor_record_offset":30852,"inlinee":7690,"source_line":1639,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::checked_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1328,"record_offset":31896,"parent_record_offset":31864,"ancestor_record_offset":30852,"inlinee":9241,"source_line":460,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::overflowing_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1328,"record_offset":31928,"parent_record_offset":31896,"ancestor_record_offset":30852,"inlinee":9242,"source_line":1975,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::expect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1328,"record_offset":31352,"parent_record_offset":31116,"ancestor_record_offset":30852,"inlinee":5324,"source_line":893,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] core::time::Duration::as_secs_f32
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\time.rs
//     evidence={"module":1328,"record_offset":31388,"parent_record_offset":31032,"ancestor_record_offset":30852,"inlinee":12632,"source_line":855,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] core::time::Duration::checked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\time.rs
//     evidence={"module":1328,"record_offset":31864,"parent_record_offset":31836,"ancestor_record_offset":30852,"inlinee":31333,"source_line":633,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] core::time::Duration::try_from_secs_f32
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\time.rs
//     evidence={"module":1328,"record_offset":31592,"parent_record_offset":31552,"ancestor_record_offset":30852,"inlinee":12634,"source_line":1602,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] core::time::impl$2::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\time.rs
//     evidence={"module":1328,"record_offset":31836,"parent_record_offset":31808,"ancestor_record_offset":30852,"inlinee":31332,"source_line":1107,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] country_core::input::input_stream::impl$2::index
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/input/input_stream.rs
//     evidence={"module":1328,"record_offset":31088,"parent_record_offset":31032,"ancestor_record_offset":30852,"inlinee":21859,"source_line":204,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::time_update::calculate_delta_seconds
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 42
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF227
// 首条位置: {"module":1328,"inlinee":61991}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::time::Duration::as_secs_f32
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\time.rs
//     evidence={"module":1328,"record_offset":31388,"parent_record_offset":31032,"ancestor_record_offset":31032,"inlinee":12632,"source_line":855,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_direct] country_core::input::input_stream::impl$2::index
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/input/input_stream.rs
//     evidence={"module":1328,"record_offset":31088,"parent_record_offset":31032,"ancestor_record_offset":31032,"inlinee":21859,"source_line":204,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1328,"record_offset":31144,"parent_record_offset":31116,"ancestor_record_offset":31032,"inlinee":7687,"source_line":667,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1328,"record_offset":31292,"parent_record_offset":31144,"ancestor_record_offset":31032,"inlinee":7712,"source_line":159,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1328,"record_offset":31176,"parent_record_offset":31144,"ancestor_record_offset":31032,"inlinee":7688,"source_line":200,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1328,"record_offset":31208,"parent_record_offset":31176,"ancestor_record_offset":31032,"inlinee":7689,"source_line":195,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::impl$12::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1328,"record_offset":31116,"parent_record_offset":31088,"ancestor_record_offset":31032,"inlinee":21861,"source_line":2771,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1328,"record_offset":31316,"parent_record_offset":31292,"ancestor_record_offset":31032,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1328,"record_offset":31240,"parent_record_offset":31208,"ancestor_record_offset":31032,"inlinee":7690,"source_line":1639,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1328,"record_offset":31352,"parent_record_offset":31116,"ancestor_record_offset":31032,"inlinee":5324,"source_line":893,"archive":"_pdb/A/modules/01328.jsonl.gz"}
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
// 完整结构及原始类型记录: time_update.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: time_update.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: time_update.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::shader_refactoring_state::ShaderRefactoringState> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::input::input_stream::InputRecordReplayState>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >", "bevy_ecs::change_detection::Res<enum2$<country_core::resources::shader_refactoring_state::ShaderRefactoringState> >", "bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >", "bevy_ecs::change_detection::Res<country_core::input::input_stream::InputRecordReplayState>", "bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::resources::shader_refactoring_state::ShaderRefactoringState> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::input_stream::InputRecordReplayState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>

// ------------------------------------------------------------------
// 函数: country_core::systems::time_update::calculate_delta_seconds
// 信息 ID: 68f63ca5ebfaf92c11176869
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::time_update::time_update; module=1328, inline_record=31032; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::time_update::time_update
// 信息 ID: fbc19b94c0e5c5c1347d617c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 0b123c8272323b63c8ea577e: void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::shader_refactoring_state::ShaderRefactoringState> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::input::input_stream::InputRecordReplayState>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::shader_refactoring_state::ShaderRefactoringState> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::input::input_stream::InputRecordReplayState>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>),void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::shader_refactoring_state::ShaderRefactoringState> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::input::input_stream::InputRecordReplayState>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>)> @ 0x6a7406 -> 0xe91d70 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::shader_refactoring_state::ShaderRefactoringState> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::input::input_stream::InputRecordReplayState>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>),void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::shader_refactoring_state::ShaderRefactoringState> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::input::input_stream::InputRecordReplayState>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>)> > @ 0xe60e88 -> 0xe91d70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
