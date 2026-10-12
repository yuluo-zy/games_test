// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/decorator/src/ui_systems/handle_splitting_wall_decorators.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 2139523b9e7faa122ed1ca53fce7ca26b7eed4f33eb3f3caf73e1827a3cb1b78
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: handle_splitting_wall_decorators.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 4379, 4555
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use smallvec; // namespace candidate
//   package identities: [{"name":"smallvec","source":"PDB_registry_path_only","version":"1.13.2"}]
// use std; // namespace candidate
//   package identities: []
// use system_decorator; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::handle_splitting_wall_decorators::display_decorator_wall_parts_and_get_hovered_part_id
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4379,"record_offset":1932,"rva":31886912,"va":5400596032}
// 编译实例 RVAs: 0x1e68e40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: handle_splitting_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27847600","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400596032,"raw_hex":"e8501457fe","size":5,"soft_fail":false,"target_va":5372749696,"va":5400597291,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: handle_splitting_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27847726","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400596032,"raw_hex":"e8d21357fe","size":5,"soft_fail":false,"target_va":5372749712,"va":5400597433,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405977c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: handle_splitting_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26024419","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400596032,"raw_hex":"e81de672fe","size":5,"soft_fail":false,"target_va":5374572480,"va":5400596894,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140633130
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: handle_splitting_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25387132","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400596032,"raw_hex":"e8849f7cfe","size":5,"soft_fail":false,"target_va":5375209776,"va":5400596903,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d06720
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: handle_splitting_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18229918","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400596032,"raw_hex":"e862d5e9fe","size":5,"soft_fail":false,"target_va":5382367008,"va":5400596921,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t2379863","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400596032,"raw_hex":"e857502400","size":5,"soft_fail":false,"target_va":5402978144,"va":5400598276,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-23067367","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400596032,"raw_hex":"e81905a0fe","size":5,"soft_fail":false,"target_va":5377529888,"va":5400597250,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2380845","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400596032,"raw_hex":"e82d542400","size":5,"soft_fail":false,"target_va":5402979152,"va":5400598302,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2380678","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400596032,"raw_hex":"e886532400","size":5,"soft_fail":false,"target_va":5402978944,"va":5400598261,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierFactory::new
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-25241621","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400596032,"raw_hex":"e8ebd77efe","size":5,"soft_fail":false,"target_va":5375355744,"va":5400597360,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::send
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-18323775","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400596032,"raw_hex":"e8c166e8fe","size":5,"soft_fail":false,"target_va":5382273632,"va":5400597402,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":4379,"record_offset":3996,"parent_record_offset":1932,"ancestor_record_offset":1932,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":4379,"record_offset":4080,"parent_record_offset":1932,"ancestor_record_offset":1932,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":4379,"record_offset":3112,"parent_record_offset":1932,"ancestor_record_offset":1932,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":4379,"record_offset":4296,"parent_record_offset":1932,"ancestor_record_offset":1932,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::dims_wall_space
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":4379,"record_offset":3196,"parent_record_offset":1932,"ancestor_record_offset":1932,"inlinee":21730,"source_line":261,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::mat3a::Mat3A::mul_vec3a
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat3a.rs
//     evidence={"module":4379,"record_offset":3568,"parent_record_offset":1932,"ancestor_record_offset":1932,"inlinee":21503,"source_line":516,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":4379,"record_offset":2184,"parent_record_offset":1932,"ancestor_record_offset":1932,"inlinee":14676,"source_line":659,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::vec3a::impl$10::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":4379,"record_offset":3484,"parent_record_offset":1932,"ancestor_record_offset":1932,"inlinee":23066,"source_line":1054,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::vec3a::impl$14::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":4379,"record_offset":3940,"parent_record_offset":1932,"ancestor_record_offset":1932,"inlinee":120874,"source_line":1084,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::vec3a::impl$47::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":4379,"record_offset":4496,"parent_record_offset":1932,"ancestor_record_offset":1932,"inlinee":23171,"source_line":1375,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":4379,"record_offset":4552,"parent_record_offset":1932,"ancestor_record_offset":1932,"inlinee":21439,"source_line":462,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":4379,"record_offset":2832,"parent_record_offset":1932,"ancestor_record_offset":1932,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":4379,"record_offset":2964,"parent_record_offset":1932,"ancestor_record_offset":1932,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":4379,"record_offset":2868,"parent_record_offset":1932,"ancestor_record_offset":1932,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4379,"record_offset":4192,"parent_record_offset":4168,"ancestor_record_offset":1932,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4379,"record_offset":4216,"parent_record_offset":4192,"ancestor_record_offset":1932,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4379,"record_offset":4448,"parent_record_offset":4424,"ancestor_record_offset":1932,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4379,"record_offset":4136,"parent_record_offset":4108,"ancestor_record_offset":1932,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4379,"record_offset":4168,"parent_record_offset":4136,"ancestor_record_offset":1932,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4379,"record_offset":4424,"parent_record_offset":4368,"ancestor_record_offset":1932,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":4379,"record_offset":4108,"parent_record_offset":4080,"ancestor_record_offset":1932,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":4379,"record_offset":4044,"parent_record_offset":4020,"ancestor_record_offset":1932,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4379,"record_offset":4396,"parent_record_offset":4368,"ancestor_record_offset":1932,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4379,"record_offset":4368,"parent_record_offset":4344,"ancestor_record_offset":1932,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":4379,"record_offset":3160,"parent_record_offset":3136,"ancestor_record_offset":1932,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":4379,"record_offset":2620,"parent_record_offset":2208,"ancestor_record_offset":1932,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":4379,"record_offset":2528,"parent_record_offset":2472,"ancestor_record_offset":1932,"inlinee":21747,"source_line":20,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":4379,"record_offset":2300,"parent_record_offset":2276,"ancestor_record_offset":1932,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":4379,"record_offset":3508,"parent_record_offset":3484,"ancestor_record_offset":1932,"inlinee":21867,"source_line":905,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":4379,"record_offset":2332,"parent_record_offset":2252,"ancestor_record_offset":1932,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":4379,"record_offset":2392,"parent_record_offset":2208,"ancestor_record_offset":1932,"inlinee":14685,"source_line":54,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":4379,"record_offset":5852,"parent_record_offset":5828,"ancestor_record_offset":1932,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":4379,"record_offset":3136,"parent_record_offset":3112,"ancestor_record_offset":1932,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":4379,"record_offset":4520,"parent_record_offset":4496,"ancestor_record_offset":1932,"inlinee":17599,"source_line":627,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":4379,"record_offset":2900,"parent_record_offset":2868,"ancestor_record_offset":1932,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":4379,"record_offset":2996,"parent_record_offset":2964,"ancestor_record_offset":1932,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":4379,"record_offset":4020,"parent_record_offset":3996,"ancestor_record_offset":1932,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":4379,"record_offset":3248,"parent_record_offset":3224,"ancestor_record_offset":1932,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":4379,"record_offset":4320,"parent_record_offset":4296,"ancestor_record_offset":1932,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":4379,"record_offset":4244,"parent_record_offset":4216,"ancestor_record_offset":1932,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorCollision::dims_wall_space
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":4379,"record_offset":3276,"parent_record_offset":3224,"ancestor_record_offset":1932,"inlinee":21733,"source_line":102,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::dims_wall_space
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":4379,"record_offset":3224,"parent_record_offset":3196,"ancestor_record_offset":1932,"inlinee":21731,"source_line":217,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$3::dims_wall_space::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":4379,"record_offset":5828,"parent_record_offset":5804,"ancestor_record_offset":1932,"inlinee":21753,"source_line":220,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":4379,"record_offset":2208,"parent_record_offset":2184,"ancestor_record_offset":1932,"inlinee":14681,"source_line":731,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":4379,"record_offset":2648,"parent_record_offset":2208,"ancestor_record_offset":1932,"inlinee":14682,"source_line":220,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":4379,"record_offset":4644,"parent_record_offset":4576,"ancestor_record_offset":1932,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":4379,"record_offset":4576,"parent_record_offset":4552,"ancestor_record_offset":1932,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":4379,"record_offset":2276,"parent_record_offset":2252,"ancestor_record_offset":1932,"inlinee":21746,"source_line":42,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":4379,"record_offset":2252,"parent_record_offset":2208,"ancestor_record_offset":1932,"inlinee":21745,"source_line":66,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] glam::swizzles::sse2::vec3a_impl::impl$0::xxx
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/swizzles/sse2/vec3a_impl.rs
//     evidence={"module":4379,"record_offset":3596,"parent_record_offset":3568,"ancestor_record_offset":1932,"inlinee":89798,"source_line":100,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] glam::swizzles::sse2::vec3a_impl::impl$0::yyy
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/swizzles/sse2/vec3a_impl.rs
//     evidence={"module":4379,"record_offset":3688,"parent_record_offset":3568,"ancestor_record_offset":1932,"inlinee":21509,"source_line":178,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] glam::swizzles::sse2::vec3a_impl::impl$0::zzz
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/swizzles/sse2/vec3a_impl.rs
//     evidence={"module":4379,"record_offset":3812,"parent_record_offset":3568,"ancestor_record_offset":1932,"inlinee":89797,"source_line":256,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":4379,"record_offset":4680,"parent_record_offset":4644,"ancestor_record_offset":1932,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle3d::height
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":4379,"record_offset":3444,"parent_record_offset":3420,"ancestor_record_offset":1932,"inlinee":21736,"source_line":65,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle3d::width
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":4379,"record_offset":3300,"parent_record_offset":3276,"ancestor_record_offset":1932,"inlinee":21735,"source_line":70,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::handle_splitting_wall_decorators::display_decorator_wall_parts_and_get_hovered_part_id::closure$0
// 记录形式: compiled_procedure; instances=1
// 源码行: 76
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4379,"record_offset":6500,"rva":31881712,"va":5400590832}
// 编译实例 RVAs: 0x1e679f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: handle_splitting_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27841202","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400590832,"raw_hex":"e84e2d57fe","size":5,"soft_fail":false,"target_va":5372749696,"va":5400590893,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: handle_splitting_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27841549","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400590832,"raw_hex":"e8f32b57fe","size":5,"soft_fail":false,"target_va":5372749712,"va":5400591256,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t2386954","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400590832,"raw_hex":"e80a6c2400","size":5,"soft_fail":false,"target_va":5402978144,"va":5400591185,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::animation_duration
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-25234329","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400590832,"raw_hex":"e867f47efe","size":5,"soft_fail":false,"target_va":5375356704,"va":5400591028,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-25234229","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400590832,"raw_hex":"e8cbf47efe","size":5,"soft_fail":false,"target_va":5375356896,"va":5400591120,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-25235028","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400590832,"raw_hex":"e8acf17efe","size":5,"soft_fail":false,"target_va":5375355984,"va":5400591007,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::position
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-25234534","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400590832,"raw_hex":"e89af37efe","size":5,"soft_fail":false,"target_va":5375356560,"va":5400591089,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::scale
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-25234398","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400590832,"raw_hex":"e822f47efe","size":5,"soft_fail":false,"target_va":5375356656,"va":5400591049,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4379,"record_offset":6724,"parent_record_offset":6500,"ancestor_record_offset":6500,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":4379,"record_offset":7428,"parent_record_offset":6500,"ancestor_record_offset":6500,"inlinee":4441,"source_line":461,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":4379,"record_offset":6872,"parent_record_offset":6500,"ancestor_record_offset":6500,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":4379,"record_offset":7512,"parent_record_offset":6500,"ancestor_record_offset":6500,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4379,"record_offset":6780,"parent_record_offset":6756,"ancestor_record_offset":6500,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4379,"record_offset":6804,"parent_record_offset":6780,"ancestor_record_offset":6500,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4379,"record_offset":7584,"parent_record_offset":7560,"ancestor_record_offset":6500,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4379,"record_offset":6756,"parent_record_offset":6724,"ancestor_record_offset":6500,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4379,"record_offset":7560,"parent_record_offset":7536,"ancestor_record_offset":6500,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":4379,"record_offset":7536,"parent_record_offset":7512,"ancestor_record_offset":6500,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4379,"record_offset":7160,"parent_record_offset":7132,"ancestor_record_offset":6500,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4379,"record_offset":7132,"parent_record_offset":7104,"ancestor_record_offset":6500,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4379,"record_offset":7304,"parent_record_offset":7132,"ancestor_record_offset":6500,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":4379,"record_offset":7068,"parent_record_offset":7040,"ancestor_record_offset":6500,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":4379,"record_offset":7452,"parent_record_offset":7428,"ancestor_record_offset":6500,"inlinee":4442,"source_line":100,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":4379,"record_offset":7040,"parent_record_offset":7012,"ancestor_record_offset":6500,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":4379,"record_offset":6984,"parent_record_offset":6956,"ancestor_record_offset":6500,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":4379,"record_offset":7012,"parent_record_offset":6984,"ancestor_record_offset":6500,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":4379,"record_offset":6956,"parent_record_offset":6928,"ancestor_record_offset":6500,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":4379,"record_offset":6928,"parent_record_offset":6900,"ancestor_record_offset":6500,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":4379,"record_offset":6900,"parent_record_offset":6872,"ancestor_record_offset":6500,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":4379,"record_offset":7476,"parent_record_offset":7452,"ancestor_record_offset":6500,"inlinee":4443,"source_line":819,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":4379,"record_offset":7104,"parent_record_offset":7068,"ancestor_record_offset":6500,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":4379,"record_offset":7364,"parent_record_offset":7340,"ancestor_record_offset":6500,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":4379,"record_offset":7340,"parent_record_offset":7068,"ancestor_record_offset":6500,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":4379,"record_offset":6828,"parent_record_offset":6804,"ancestor_record_offset":6500,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::handle_splitting_wall_decorators::display_decorator_wall_parts_and_get_hovered_part_id::closure$1
// 记录形式: compiled_procedure; instances=1
// 源码行: 104
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4379,"record_offset":7628,"rva":31882160,"va":5400591280}
// 编译实例 RVAs: 0x1e67bb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: handle_splitting_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27841650","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400591280,"raw_hex":"e88e2b57fe","size":5,"soft_fail":false,"target_va":5372749696,"va":5400591341,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: handle_splitting_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27841997","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400591280,"raw_hex":"e8332a57fe","size":5,"soft_fail":false,"target_va":5372749712,"va":5400591704,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t2386506","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400591280,"raw_hex":"e84a6a2400","size":5,"soft_fail":false,"target_va":5402978144,"va":5400591633,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::animation_duration
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-25234777","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400591280,"raw_hex":"e8a7f27efe","size":5,"soft_fail":false,"target_va":5375356704,"va":5400591476,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-25234677","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400591280,"raw_hex":"e80bf37efe","size":5,"soft_fail":false,"target_va":5375356896,"va":5400591568,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-25235476","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400591280,"raw_hex":"e8ecef7efe","size":5,"soft_fail":false,"target_va":5375355984,"va":5400591455,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::position
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-25234982","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400591280,"raw_hex":"e8daf17efe","size":5,"soft_fail":false,"target_va":5375356560,"va":5400591537,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::scale
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-25234846","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400591280,"raw_hex":"e862f27efe","size":5,"soft_fail":false,"target_va":5375356656,"va":5400591497,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4379,"record_offset":7852,"parent_record_offset":7628,"ancestor_record_offset":7628,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":4379,"record_offset":8556,"parent_record_offset":7628,"ancestor_record_offset":7628,"inlinee":4441,"source_line":461,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":4379,"record_offset":8000,"parent_record_offset":7628,"ancestor_record_offset":7628,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":4379,"record_offset":8640,"parent_record_offset":7628,"ancestor_record_offset":7628,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4379,"record_offset":7908,"parent_record_offset":7884,"ancestor_record_offset":7628,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4379,"record_offset":7932,"parent_record_offset":7908,"ancestor_record_offset":7628,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4379,"record_offset":8712,"parent_record_offset":8688,"ancestor_record_offset":7628,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4379,"record_offset":7884,"parent_record_offset":7852,"ancestor_record_offset":7628,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4379,"record_offset":8688,"parent_record_offset":8664,"ancestor_record_offset":7628,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":4379,"record_offset":8664,"parent_record_offset":8640,"ancestor_record_offset":7628,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4379,"record_offset":8288,"parent_record_offset":8260,"ancestor_record_offset":7628,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4379,"record_offset":8260,"parent_record_offset":8232,"ancestor_record_offset":7628,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4379,"record_offset":8432,"parent_record_offset":8260,"ancestor_record_offset":7628,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":4379,"record_offset":8196,"parent_record_offset":8168,"ancestor_record_offset":7628,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":4379,"record_offset":8580,"parent_record_offset":8556,"ancestor_record_offset":7628,"inlinee":4442,"source_line":100,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":4379,"record_offset":8168,"parent_record_offset":8140,"ancestor_record_offset":7628,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":4379,"record_offset":8112,"parent_record_offset":8084,"ancestor_record_offset":7628,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":4379,"record_offset":8140,"parent_record_offset":8112,"ancestor_record_offset":7628,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":4379,"record_offset":8084,"parent_record_offset":8056,"ancestor_record_offset":7628,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":4379,"record_offset":8056,"parent_record_offset":8028,"ancestor_record_offset":7628,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":4379,"record_offset":8028,"parent_record_offset":8000,"ancestor_record_offset":7628,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":4379,"record_offset":8604,"parent_record_offset":8580,"ancestor_record_offset":7628,"inlinee":4443,"source_line":819,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":4379,"record_offset":8232,"parent_record_offset":8196,"ancestor_record_offset":7628,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":4379,"record_offset":8492,"parent_record_offset":8468,"ancestor_record_offset":7628,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":4379,"record_offset":8468,"parent_record_offset":8196,"ancestor_record_offset":7628,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":4379,"record_offset":7956,"parent_record_offset":7932,"ancestor_record_offset":7628,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::handle_splitting_wall_decorators::display_decorator_wall_parts_and_get_hovered_part_id::closure$2
// 记录形式: compiled_procedure; instances=1
// 源码行: 121
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4379,"record_offset":8756,"rva":31882608,"va":5400591728}
// 编译实例 RVAs: 0x1e67d70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: handle_splitting_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27842098","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400591728,"raw_hex":"e8ce2957fe","size":5,"soft_fail":false,"target_va":5372749696,"va":5400591789,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: handle_splitting_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27842445","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400591728,"raw_hex":"e8732857fe","size":5,"soft_fail":false,"target_va":5372749712,"va":5400592152,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t2386058","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400591728,"raw_hex":"e88a682400","size":5,"soft_fail":false,"target_va":5402978144,"va":5400592081,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::animation_duration
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-25235225","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400591728,"raw_hex":"e8e7f07efe","size":5,"soft_fail":false,"target_va":5375356704,"va":5400591924,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-25235125","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400591728,"raw_hex":"e84bf17efe","size":5,"soft_fail":false,"target_va":5375356896,"va":5400592016,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-25235924","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400591728,"raw_hex":"e82cee7efe","size":5,"soft_fail":false,"target_va":5375355984,"va":5400591903,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::position
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-25235430","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400591728,"raw_hex":"e81af07efe","size":5,"soft_fail":false,"target_va":5375356560,"va":5400591985,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::scale
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-25235294","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400591728,"raw_hex":"e8a2f07efe","size":5,"soft_fail":false,"target_va":5375356656,"va":5400591945,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4379,"record_offset":8980,"parent_record_offset":8756,"ancestor_record_offset":8756,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":4379,"record_offset":9684,"parent_record_offset":8756,"ancestor_record_offset":8756,"inlinee":4441,"source_line":461,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":4379,"record_offset":9128,"parent_record_offset":8756,"ancestor_record_offset":8756,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":4379,"record_offset":9768,"parent_record_offset":8756,"ancestor_record_offset":8756,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4379,"record_offset":9036,"parent_record_offset":9012,"ancestor_record_offset":8756,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4379,"record_offset":9060,"parent_record_offset":9036,"ancestor_record_offset":8756,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4379,"record_offset":9840,"parent_record_offset":9816,"ancestor_record_offset":8756,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4379,"record_offset":9012,"parent_record_offset":8980,"ancestor_record_offset":8756,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4379,"record_offset":9816,"parent_record_offset":9792,"ancestor_record_offset":8756,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":4379,"record_offset":9792,"parent_record_offset":9768,"ancestor_record_offset":8756,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4379,"record_offset":9416,"parent_record_offset":9388,"ancestor_record_offset":8756,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4379,"record_offset":9388,"parent_record_offset":9360,"ancestor_record_offset":8756,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4379,"record_offset":9560,"parent_record_offset":9388,"ancestor_record_offset":8756,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":4379,"record_offset":9324,"parent_record_offset":9296,"ancestor_record_offset":8756,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":4379,"record_offset":9708,"parent_record_offset":9684,"ancestor_record_offset":8756,"inlinee":4442,"source_line":100,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":4379,"record_offset":9296,"parent_record_offset":9268,"ancestor_record_offset":8756,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":4379,"record_offset":9240,"parent_record_offset":9212,"ancestor_record_offset":8756,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":4379,"record_offset":9268,"parent_record_offset":9240,"ancestor_record_offset":8756,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":4379,"record_offset":9212,"parent_record_offset":9184,"ancestor_record_offset":8756,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":4379,"record_offset":9184,"parent_record_offset":9156,"ancestor_record_offset":8756,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":4379,"record_offset":9156,"parent_record_offset":9128,"ancestor_record_offset":8756,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":4379,"record_offset":9732,"parent_record_offset":9708,"ancestor_record_offset":8756,"inlinee":4443,"source_line":819,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":4379,"record_offset":9360,"parent_record_offset":9324,"ancestor_record_offset":8756,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":4379,"record_offset":9620,"parent_record_offset":9596,"ancestor_record_offset":8756,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":4379,"record_offset":9596,"parent_record_offset":9324,"ancestor_record_offset":8756,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":4379,"record_offset":9084,"parent_record_offset":9060,"ancestor_record_offset":8756,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::handle_splitting_wall_decorators::handle_moving_out_wall_decorator_part
// 记录形式: compiled_procedure; instances=1
// 源码行: 140
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4379,"record_offset":9884,"rva":31889904,"va":5400599024}
// 编译实例 RVAs: 0x1e699f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140004020
//     owner=unresolved_namespace; count=2
//     target_aliases=154 candidates; complete list: handle_splitting_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-31874812","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400599024,"raw_hex":"e804a119fe","size":5,"soft_fail":false,"target_va":5368725536,"va":5400600343,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: handle_splitting_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-30985687","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400599024,"raw_hex":"e8293227fe","size":5,"soft_fail":false,"target_va":5369613520,"va":5400599202,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: handle_splitting_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27852629","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400599024,"raw_hex":"e8ab0057fe","size":5,"soft_fail":false,"target_va":5372749712,"va":5400602336,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140593250
//     owner=unresolved_namespace; count=3
//     target_aliases=15 candidates; complete list: handle_splitting_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26045026","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400599024,"raw_hex":"e89e9572fe","size":5,"soft_fail":false,"target_va":5374554704,"va":5400599725,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405baba0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: handle_splitting_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25882333","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400599024,"raw_hex":"e8231175fe","size":5,"soft_fail":false,"target_va":5374716832,"va":5400599160,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140633130
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: handle_splitting_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25389982","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400599024,"raw_hex":"e862947cfe","size":5,"soft_fail":false,"target_va":5375209776,"va":5400599753,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d01eb0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: handle_splitting_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18254526","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400599024,"raw_hex":"e84275e9fe","size":5,"soft_fail":false,"target_va":5382348464,"va":5400602985,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d06720
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: handle_splitting_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18232788","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400599024,"raw_hex":"e82ccae9fe","size":5,"soft_fail":false,"target_va":5382367008,"va":5400599791,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-29295972","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400599024,"raw_hex":"e89cfa40fe","size":5,"soft_fail":false,"target_va":5371305056,"va":5400601023,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t2376450","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400599024,"raw_hex":"e802432400","size":5,"soft_fail":false,"target_va":5402978880,"va":5400602425,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2376291","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400599024,"raw_hex":"e863422400","size":5,"soft_fail":false,"target_va":5402979152,"va":5400602856,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t2376387","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400599024,"raw_hex":"e8c3422400","size":5,"soft_fail":false,"target_va":5402978944,"va":5400602552,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorInfo::try_dst_as_wall
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-26028323","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400599024,"raw_hex":"e8ddd672fe","size":5,"soft_fail":false,"target_va":5374571104,"va":5400599422,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::TransientDecoratorIdGen::generate
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-26029025","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400599024,"raw_hex":"e81fd472fe","size":5,"soft_fail":false,"target_va":5374572432,"va":5400601452,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::subtype
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-25386996","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400599024,"raw_hex":"e80ca07cfe","size":5,"soft_fail":false,"target_va":5375212688,"va":5400599679,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::wall_attachment::WallCornerAttachment::is_attached
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-25163008","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400599024,"raw_hex":"e8000b80fe","size":5,"soft_fail":false,"target_va":5375436576,"va":5400599579,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] smallvec::SmallVec$LT$A$GT$::reserve_one_unchecked
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2990734","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400599024,"raw_hex":"e88ea22d00","size":5,"soft_fail":false,"target_va":5403590992,"va":5400600253,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::apply_decorator_op::apply_decorator_op
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-18235782","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400599024,"raw_hex":"e87abee9fe","size":5,"soft_fail":false,"target_va":5382365568,"va":5400601345,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::ui_systems::handle_splitting_wall_decorators::handle_moving_out_wall_decorator_part::closure$3
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t2160","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400599024,"raw_hex":"e870080000","size":5,"soft_fail":false,"target_va":5400603280,"va":5400601115,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":4379,"record_offset":13472,"parent_record_offset":9884,"ancestor_record_offset":9884,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":4379,"record_offset":13604,"parent_record_offset":9884,"ancestor_record_offset":9884,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::filter::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\filter.rs
//     evidence={"module":4379,"record_offset":11820,"parent_record_offset":9884,"ancestor_record_offset":9884,"inlinee":8575,"source_line":59,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":4379,"record_offset":11344,"parent_record_offset":9884,"ancestor_record_offset":9884,"inlinee":4983,"source_line":604,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":4379,"record_offset":11188,"parent_record_offset":9884,"ancestor_record_offset":9884,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":4379,"record_offset":14020,"parent_record_offset":9884,"ancestor_record_offset":9884,"inlinee":4710,"source_line":651,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":4379,"record_offset":11504,"parent_record_offset":9884,"ancestor_record_offset":9884,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":4379,"record_offset":11224,"parent_record_offset":9884,"ancestor_record_offset":9884,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":4379,"record_offset":11284,"parent_record_offset":9884,"ancestor_record_offset":9884,"inlinee":4773,"source_line":1127,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":4379,"record_offset":10152,"parent_record_offset":9884,"ancestor_record_offset":9884,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":4379,"record_offset":14104,"parent_record_offset":9884,"ancestor_record_offset":9884,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::dims_wall_space
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":4379,"record_offset":11532,"parent_record_offset":9884,"ancestor_record_offset":9884,"inlinee":21730,"source_line":261,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$10::mul_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":4379,"record_offset":13176,"parent_record_offset":9884,"ancestor_record_offset":9884,"inlinee":14658,"source_line":982,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":4379,"record_offset":11756,"parent_record_offset":9884,"ancestor_record_offset":9884,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$8::mul_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":4379,"record_offset":13120,"parent_record_offset":9884,"ancestor_record_offset":9884,"inlinee":23596,"source_line":963,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":4379,"record_offset":12448,"parent_record_offset":9884,"ancestor_record_offset":9884,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_direct] smallvec::SmallVec::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":4379,"record_offset":11256,"parent_record_offset":9884,"ancestor_record_offset":9884,"inlinee":21555,"source_line":779,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_direct] smallvec::SmallVec::push
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"smallvec","version":"1.13.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":4379,"record_offset":11444,"parent_record_offset":9884,"ancestor_record_offset":9884,"inlinee":21554,"source_line":1114,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_direct] smallvec::impl$44::next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"smallvec","version":"1.13.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":4379,"record_offset":13500,"parent_record_offset":9884,"ancestor_record_offset":9884,"inlinee":34931,"source_line":2237,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_direct] smallvec::impl$49::into_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":4379,"record_offset":13232,"parent_record_offset":9884,"ancestor_record_offset":9884,"inlinee":34978,"source_line":2290,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":4379,"record_offset":10188,"parent_record_offset":9884,"ancestor_record_offset":9884,"inlinee":5613,"source_line":1070,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4379,"record_offset":14456,"parent_record_offset":14432,"ancestor_record_offset":9884,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4379,"record_offset":14432,"parent_record_offset":14344,"ancestor_record_offset":9884,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4379,"record_offset":14372,"parent_record_offset":14344,"ancestor_record_offset":9884,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4379,"record_offset":13660,"parent_record_offset":13636,"ancestor_record_offset":9884,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4379,"record_offset":14344,"parent_record_offset":14320,"ancestor_record_offset":9884,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":4379,"record_offset":13636,"parent_record_offset":13604,"ancestor_record_offset":9884,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\iter.rs
//     evidence={"module":4379,"record_offset":11896,"parent_record_offset":11868,"ancestor_record_offset":9884,"inlinee":6222,"source_line":239,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\iter.rs
//     evidence={"module":4379,"record_offset":11984,"parent_record_offset":11920,"ancestor_record_offset":9884,"inlinee":11996,"source_line":245,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":4379,"record_offset":10972,"parent_record_offset":10948,"ancestor_record_offset":9884,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":4379,"record_offset":10440,"parent_record_offset":10416,"ancestor_record_offset":9884,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":4379,"record_offset":10524,"parent_record_offset":10500,"ancestor_record_offset":9884,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":4379,"record_offset":11656,"parent_record_offset":11632,"ancestor_record_offset":9884,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":4379,"record_offset":10464,"parent_record_offset":10440,"ancestor_record_offset":9884,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":4379,"record_offset":11844,"parent_record_offset":11820,"ancestor_record_offset":9884,"inlinee":6348,"source_line":2866,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":4379,"record_offset":12104,"parent_record_offset":11868,"ancestor_record_offset":9884,"inlinee":6192,"source_line":2873,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":4379,"record_offset":11868,"parent_record_offset":11844,"ancestor_record_offset":9884,"inlinee":4926,"source_line":2401,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$0::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":4379,"record_offset":14156,"parent_record_offset":14132,"ancestor_record_offset":9884,"inlinee":6886,"source_line":4105,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":4379,"record_offset":14272,"parent_record_offset":14244,"ancestor_record_offset":9884,"inlinee":5355,"source_line":992,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init_read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":4379,"record_offset":12008,"parent_record_offset":11984,"ancestor_record_offset":9884,"inlinee":6097,"source_line":697,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":4379,"record_offset":11372,"parent_record_offset":11344,"ancestor_record_offset":9884,"inlinee":4984,"source_line":2074,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":4379,"record_offset":14396,"parent_record_offset":14372,"ancestor_record_offset":9884,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":4379,"record_offset":10636,"parent_record_offset":10612,"ancestor_record_offset":9884,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":4379,"record_offset":12472,"parent_record_offset":12448,"ancestor_record_offset":9884,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$364::mul_assign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":4379,"record_offset":13144,"parent_record_offset":13120,"ancestor_record_offset":9884,"inlinee":14660,"source_line":886,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":4379,"record_offset":11784,"parent_record_offset":11756,"ancestor_record_offset":9884,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":4379,"record_offset":12128,"parent_record_offset":12104,"ancestor_record_offset":9884,"inlinee":4399,"source_line":293,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\index_range.rs
//     evidence={"module":4379,"record_offset":11956,"parent_record_offset":11920,"ancestor_record_offset":9884,"inlinee":6100,"source_line":54,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\index_range.rs
//     evidence={"module":4379,"record_offset":11920,"parent_record_offset":11896,"ancestor_record_offset":9884,"inlinee":6069,"source_line":118,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":4379,"record_offset":14044,"parent_record_offset":14020,"ancestor_record_offset":9884,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":4379,"record_offset":11600,"parent_record_offset":11572,"ancestor_record_offset":9884,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":4379,"record_offset":13760,"parent_record_offset":13724,"ancestor_record_offset":9884,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":4379,"record_offset":12032,"parent_record_offset":12008,"ancestor_record_offset":9884,"inlinee":6098,"source_line":1250,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":4379,"record_offset":14220,"parent_record_offset":14104,"ancestor_record_offset":9884,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4379,"record_offset":12640,"parent_record_offset":12504,"ancestor_record_offset":9884,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4379,"record_offset":10884,"parent_record_offset":10860,"ancestor_record_offset":9884,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4379,"record_offset":10776,"parent_record_offset":10752,"ancestor_record_offset":9884,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":4379,"record_offset":12056,"parent_record_offset":12032,"ancestor_record_offset":9884,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":4379,"record_offset":11472,"parent_record_offset":11444,"ancestor_record_offset":9884,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$25::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":4379,"record_offset":10996,"parent_record_offset":10972,"ancestor_record_offset":9884,"inlinee":21651,"source_line":84,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorCollision::dims_wall_space
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":4379,"record_offset":11692,"parent_record_offset":11572,"ancestor_record_offset":9884,"inlinee":21733,"source_line":102,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::dims_wall_space
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":4379,"record_offset":11572,"parent_record_offset":11532,"ancestor_record_offset":9884,"inlinee":21731,"source_line":217,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$3::dims_wall_space::closure$0
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":4379,"record_offset":11632,"parent_record_offset":11600,"ancestor_record_offset":9884,"inlinee":21753,"source_line":220,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":4379,"record_offset":10948,"parent_record_offset":10924,"ancestor_record_offset":9884,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":4379,"record_offset":10236,"parent_record_offset":10212,"ancestor_record_offset":9884,"inlinee":5615,"source_line":1461,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":4379,"record_offset":10212,"parent_record_offset":10188,"ancestor_record_offset":9884,"inlinee":5614,"source_line":1449,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":4379,"record_offset":10924,"parent_record_offset":10704,"ancestor_record_offset":9884,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":4379,"record_offset":10268,"parent_record_offset":10236,"ancestor_record_offset":9884,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":4379,"record_offset":10836,"parent_record_offset":10812,"ancestor_record_offset":9884,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":4379,"record_offset":10812,"parent_record_offset":10704,"ancestor_record_offset":9884,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":4379,"record_offset":10752,"parent_record_offset":10728,"ancestor_record_offset":9884,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":4379,"record_offset":11132,"parent_record_offset":10344,"ancestor_record_offset":9884,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":4379,"record_offset":10728,"parent_record_offset":10704,"ancestor_record_offset":9884,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":4379,"record_offset":10320,"parent_record_offset":10296,"ancestor_record_offset":9884,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":4379,"record_offset":10296,"parent_record_offset":10236,"ancestor_record_offset":9884,"inlinee":5619,"source_line":1489,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":4379,"record_offset":10344,"parent_record_offset":10320,"ancestor_record_offset":9884,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":4379,"record_offset":10584,"parent_record_offset":10556,"ancestor_record_offset":9884,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":4379,"record_offset":10612,"parent_record_offset":10584,"ancestor_record_offset":9884,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":4379,"record_offset":10672,"parent_record_offset":10556,"ancestor_record_offset":9884,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":4379,"record_offset":10556,"parent_record_offset":10344,"ancestor_record_offset":9884,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":4379,"record_offset":10388,"parent_record_offset":10344,"ancestor_record_offset":9884,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":4379,"record_offset":10704,"parent_record_offset":10344,"ancestor_record_offset":9884,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":4379,"record_offset":10416,"parent_record_offset":10344,"ancestor_record_offset":9884,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":4379,"record_offset":10500,"parent_record_offset":10344,"ancestor_record_offset":9884,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":4379,"record_offset":11040,"parent_record_offset":10344,"ancestor_record_offset":9884,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::as_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":4379,"record_offset":13536,"parent_record_offset":13500,"ancestor_record_offset":9884,"inlinee":59979,"source_line":1716,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":4379,"record_offset":13284,"parent_record_offset":13232,"ancestor_record_offset":9884,"inlinee":34970,"source_line":952,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::set_len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":4379,"record_offset":13440,"parent_record_offset":13232,"ancestor_record_offset":9884,"inlinee":34979,"source_line":919,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::spilled
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"smallvec","version":"1.13.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":4379,"record_offset":12576,"parent_record_offset":12552,"ancestor_record_offset":9884,"inlinee":18839,"source_line":1001,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::triple
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"smallvec","version":"1.13.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":4379,"record_offset":13332,"parent_record_offset":13284,"ancestor_record_offset":9884,"inlinee":34971,"source_line":971,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::triple_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":4379,"record_offset":12552,"parent_record_offset":12504,"ancestor_record_offset":9884,"inlinee":21556,"source_line":984,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVecData::heap_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"smallvec","version":"1.13.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":4379,"record_offset":12612,"parent_record_offset":12504,"ancestor_record_offset":9884,"inlinee":21558,"source_line":727,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::impl$33::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"smallvec","version":"1.13.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":4379,"record_offset":14244,"parent_record_offset":14220,"ancestor_record_offset":9884,"inlinee":18837,"source_line":2118,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::impl$43::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":4379,"record_offset":14132,"parent_record_offset":14104,"ancestor_record_offset":9884,"inlinee":34929,"source_line":2228,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::impl$44::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":4379,"record_offset":14180,"parent_record_offset":14156,"ancestor_record_offset":9884,"inlinee":34931,"source_line":2237,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::ui_systems::handle_splitting_wall_decorators::handle_moving_out_wall_decorator_part::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/handle_splitting_wall_decorators.rs
//     evidence={"module":4379,"record_offset":11312,"parent_record_offset":11284,"ancestor_record_offset":9884,"inlinee":120876,"source_line":195,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::ui_systems::handle_splitting_wall_decorators::handle_moving_out_wall_decorator_part::closure$2
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/handle_splitting_wall_decorators.rs
//     evidence={"module":4379,"record_offset":12152,"parent_record_offset":12128,"ancestor_record_offset":9884,"inlinee":120877,"source_line":212,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle3d::height
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":4379,"record_offset":11716,"parent_record_offset":11692,"ancestor_record_offset":9884,"inlinee":21736,"source_line":65,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle3d::width
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":4379,"record_offset":12380,"parent_record_offset":12356,"ancestor_record_offset":9884,"inlinee":21735,"source_line":70,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::handle_splitting_wall_decorators::handle_moving_out_wall_decorator_part::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 195
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1D82C
// 首条位置: {"module":4379,"inlinee":120876}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::handle_splitting_wall_decorators::handle_moving_out_wall_decorator_part::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 212
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1D82D
// 首条位置: {"module":4379,"inlinee":120877}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::handle_splitting_wall_decorators::handle_moving_out_wall_decorator_part::closure$3
// 记录形式: compiled_procedure; instances=1
// 源码行: 255
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4379,"record_offset":15116,"rva":31894160,"va":5400603280}
// 编译实例 RVAs: 0x1e6aa90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140003390
//     owner=unresolved_namespace; count=1
//     target_aliases=1000 candidates; complete list: handle_splitting_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-31881209","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400603280,"raw_hex":"e8078819fe","size":5,"soft_fail":false,"target_va":5368722320,"va":5400603524,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ceb4e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: handle_splitting_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18347627","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400603280,"raw_hex":"e89509e8fe","size":5,"soft_fail":false,"target_va":5382255840,"va":5400603462,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t2375033","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400603280,"raw_hex":"e8793d2400","size":5,"soft_fail":false,"target_va":5402978880,"va":5400603842,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::insert
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-26240779","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400603280,"raw_hex":"e8f5986ffe","size":5,"soft_fail":false,"target_va":5374362928,"va":5400603702,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorageAddr::try_as_wall
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-25886567","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400603280,"raw_hex":"e8990075fe","size":5,"soft_fail":false,"target_va":5374716784,"va":5400603346,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::get_wall_attachment_height
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-18670176","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400603280,"raw_hex":"e8a01de3fe","size":5,"soft_fail":false,"target_va":5381933344,"va":5400603515,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::wall_space
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-18670388","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400603280,"raw_hex":"e8cc1ce3fe","size":5,"soft_fail":false,"target_va":5381933056,"va":5400603439,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-25168399","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400603280,"raw_hex":"e8f1f57ffe","size":5,"soft_fail":false,"target_va":5375434976,"va":5400603370,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::wall_space::WallSpaceCoord::get_tangent
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-18346287","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400603280,"raw_hex":"e8d10ee8fe","size":5,"soft_fail":false,"target_va":5382257360,"va":5400603642,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::wall_space::WallSpaceCoord::to_world_space
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-18346985","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400603280,"raw_hex":"e8170ce8fe","size":5,"soft_fail":false,"target_va":5382256592,"va":5400603572,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::xz_rotation
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-23660726","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400603280,"raw_hex":"e84af796fe","size":5,"soft_fail":false,"target_va":5376942944,"va":5400603665,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":4379,"record_offset":15324,"parent_record_offset":15116,"ancestor_record_offset":15116,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":4379,"record_offset":15432,"parent_record_offset":15116,"ancestor_record_offset":15116,"inlinee":4156,"source_line":1003,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":4379,"record_offset":15604,"parent_record_offset":15116,"ancestor_record_offset":15116,"inlinee":14690,"source_line":814,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_direct] utils::wall_space::WallSpaceCoord::with_offset
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":4379,"record_offset":15468,"parent_record_offset":15116,"ancestor_record_offset":15116,"inlinee":21743,"source_line":197,"archive":"_pdb/A/modules/04379.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$40::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":4379,"record_offset":15500,"parent_record_offset":15468,"ancestor_record_offset":15116,"inlinee":5366,"source_line":2485,"archive":"_pdb/A/modules/04379.jsonl.gz"}
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
// 完整结构及原始类型记录: handle_splitting_wall_decorators.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_decorator::ui_systems::handle_splitting_wall_decorators::handle_moving_out_wall_decorator_part::FracturedPart
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "array$<system_decorator::ui_systems::handle_splitting_wall_decorators::handle_moving_out_wall_decorator_part::FracturedPart,2>", "constructor": "array$", "arguments": ["system_decorator::ui_systems::handle_splitting_wall_decorators::handle_moving_out_wall_decorator_part::FracturedPart", "2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "smallvec::SmallVec::reserve_one_unchecked<array$<system_decorator::ui_systems::handle_splitting_wall_decorators::handle_moving_out_wall_decorator_part::FracturedPart,2> >", "constructor": "smallvec::SmallVec::reserve_one_unchecked", "arguments": ["array$<system_decorator::ui_systems::handle_splitting_wall_decorators::handle_moving_out_wall_decorator_part::FracturedPart,2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: handle_splitting_wall_decorators.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: handle_splitting_wall_decorators.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::handle_splitting_wall_decorators::display_decorator_wall_parts_and_get_hovered_part_id
// 信息 ID: aef78bb7d668adad9662bd58
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::ui_move_decorator::ui_move_decorator @ 0x1c77511 -> 0x1e68e40 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::handle_splitting_wall_decorators::display_decorator_wall_parts_and_get_hovered_part_id::closure$0
// 信息 ID: e54de81a6eb2878754e4f939
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::ui_systems::handle_splitting_wall_decorators::display_decorator_wall_parts_and_get_hovered_part_id", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::ui_systems::handle_splitting_wall_decorators::display_decorator_wall_parts_and_get_hovered_part_id::closure_env$0 <- core::ops::function::FnOnce::call_once<system_decorator::ui_systems::handle_splitting_wall_decorators::display_decorator_wall_parts_and_get_hovered_part_id::closure_env$0,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::ui_systems::handle_splitting_wall_decorators::display_decorator_wall_parts_and_get_hovered_part_id::closure_env$0 <- core::ptr::drop_in_place<system_decorator::ui_systems::handle_splitting_wall_decorators::display_decorator_wall_parts_and_get_hovered_part_id::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 2 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id::closure_env$1,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > | core::ops::function::FnOnce::call_once<system_decorator::ui_systems::handle_splitting_wall_decorators::display_decorator_wall_parts_and_get_hovered_part_id::closure_env$0,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0x1e66c02 -> 0x1e679f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::handle_splitting_wall_decorators::display_decorator_wall_parts_and_get_hovered_part_id::closure$1
// 信息 ID: e28d99c9474195e536e2a062
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_decorator::ui_systems::handle_splitting_wall_decorators::display_decorator_wall_parts_and_get_hovered_part_id", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::ui_systems::handle_splitting_wall_decorators::display_decorator_wall_parts_and_get_hovered_part_id::closure_env$1 <- core::ops::function::FnOnce::call_once<system_decorator::ui_systems::handle_splitting_wall_decorators::display_decorator_wall_parts_and_get_hovered_part_id::closure_env$1,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id::closure_env$2,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > | core::ops::function::FnOnce::call_once<system_decorator::ui_systems::handle_splitting_wall_decorators::display_decorator_wall_parts_and_get_hovered_part_id::closure_env$1,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0x1e66c22 -> 0x1e67bb0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::handle_splitting_wall_decorators::display_decorator_wall_parts_and_get_hovered_part_id::closure$2
// 信息 ID: eae174f99816bd04131027cd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_decorator::ui_systems::handle_splitting_wall_decorators::display_decorator_wall_parts_and_get_hovered_part_id", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::ui_systems::handle_splitting_wall_decorators::display_decorator_wall_parts_and_get_hovered_part_id::closure_env$2 <- core::ops::function::FnOnce::call_once<system_decorator::ui_systems::handle_splitting_wall_decorators::display_decorator_wall_parts_and_get_hovered_part_id::closure_env$2,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id::closure_env$3,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > | core::ops::function::FnOnce::call_once<system_decorator::ui_systems::handle_splitting_wall_decorators::display_decorator_wall_parts_and_get_hovered_part_id::closure_env$2,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0x1e66c42 -> 0x1e67d70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::handle_splitting_wall_decorators::handle_moving_out_wall_decorator_part
// 信息 ID: d7f390c4191b27947a552909
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::ui_move_decorator::ui_move_decorator @ 0x1c77410 -> 0x1e699f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::handle_splitting_wall_decorators::handle_moving_out_wall_decorator_part::closure$1
// 信息 ID: 5c2def4a8dd837c8ab7d284e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_decorator::ui_systems::handle_splitting_wall_decorators::handle_moving_out_wall_decorator_part", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::ui_systems::handle_splitting_wall_decorators::handle_moving_out_wall_decorator_part; module=4379, inline_record=11312; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::handle_splitting_wall_decorators::handle_moving_out_wall_decorator_part::closure$2
// 信息 ID: 54666da4226f79de037bd881
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_decorator::ui_systems::handle_splitting_wall_decorators::handle_moving_out_wall_decorator_part", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::ui_systems::handle_splitting_wall_decorators::handle_moving_out_wall_decorator_part; module=4379, inline_record=12152; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::handle_splitting_wall_decorators::handle_moving_out_wall_decorator_part::closure$3
// 信息 ID: b0807c3e611634ff9c132c02
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "system_decorator::ui_systems::handle_splitting_wall_decorators::handle_moving_out_wall_decorator_part", "captures": "unknown"}
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::handle_splitting_wall_decorators::handle_moving_out_wall_decorator_part @ 0x1e6a21b -> 0x1e6aa90 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::handle_splitting_wall_decorators::handle_moving_out_wall_decorator_part @ 0x1e6a3d6 -> 0x1e6aa90 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::handle_splitting_wall_decorators::handle_moving_out_wall_decorator_part @ 0x1e6a551 -> 0x1e6aa90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
