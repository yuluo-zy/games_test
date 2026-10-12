// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/wall-constructor/src/utils/resolve_hole_overlap.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 7bd2071b14a316f8358f39f2308dd7343ba16fc32d911c83d489cdec483f4635
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: resolve_hole_overlap.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 2720
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use system_wall_constructor; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::resolve_hole_overlap::resolve_hole_overlap
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2720,"record_offset":260,"rva":24634688,"va":5393343808}
// 编译实例 RVAs: 0x177e540
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1407d9780
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: resolve_hole_overlap.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16405080","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393343808,"raw_hex":"e8a8ad05ff","size":5,"soft_fail":false,"target_va":5376939904,"va":5393344979,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::collections::vec_deque::VecDeque$LT$T$C$A$GT$::grow::h99d40cc3cf00a19d (.llvm.5524627774732696930)
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t14791","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393343808,"raw_hex":"e8c7390000","size":5,"soft_fail":false,"target_va":5393360608,"va":5393345812,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t9630323","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393343808,"raw_hex":"e873f29200","size":5,"soft_fail":false,"target_va":5402978912,"va":5393348584,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t9630635","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393343808,"raw_hex":"e8abf39200","size":5,"soft_fail":false,"target_va":5402979152,"va":5393348512,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::center
//     owner=unresolved_namespace; count=10
//     evidence={"assembly":"\tcallq\t-8924029","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393343808,"raw_hex":"e883d477ff","size":5,"soft_fail":false,"target_va":5384420192,"va":5393344216,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::contains
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-8924072","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393343808,"raw_hex":"e858d477ff","size":5,"soft_fail":false,"target_va":5384420320,"va":5393344387,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::from_center_dims
//     owner=unresolved_namespace; count=10
//     evidence={"assembly":"\tcallq\t-8925644","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393343808,"raw_hex":"e834ce77ff","size":5,"soft_fail":false,"target_va":5384420000,"va":5393345639,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::height
//     owner=unresolved_namespace; count=7
//     evidence={"assembly":"\tcallq\t-8923204","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393343808,"raw_hex":"e8bcd777ff","size":5,"soft_fail":false,"target_va":5384420752,"va":5393343951,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::width
//     owner=unresolved_namespace; count=7
//     evidence={"assembly":"\tcallq\t-8923195","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393343808,"raw_hex":"e8c5d777ff","size":5,"soft_fail":false,"target_va":5384420736,"va":5393343926,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":2720,"record_offset":440,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":7687,"source_line":667,"archive":"_pdb/A/modules/02720.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::push_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":2720,"record_offset":2612,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":10291,"source_line":1685,"archive":"_pdb/A/modules/02720.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::push_front
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":2720,"record_offset":4084,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":7709,"source_line":1658,"archive":"_pdb/A/modules/02720.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::swap_remove_back
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":2720,"record_offset":668,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":89644,"source_line":1760,"archive":"_pdb/A/modules/02720.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::impl$13::index_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":2720,"record_offset":2340,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":24158,"source_line":2779,"archive":"_pdb/A/modules/02720.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::utils::resolve_hole_overlap::check_for_hole_overlap
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/resolve_hole_overlap.rs
//     evidence={"module":2720,"record_offset":724,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":89646,"source_line":237,"archive":"_pdb/A/modules/02720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::buffer_write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":2720,"record_offset":2824,"parent_record_offset":2612,"ancestor_record_offset":260,"inlinee":6132,"source_line":171,"archive":"_pdb/A/modules/02720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::capacity
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":2720,"record_offset":2668,"parent_record_offset":2644,"ancestor_record_offset":260,"inlinee":7711,"source_line":751,"archive":"_pdb/A/modules/02720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::get_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":2720,"record_offset":2368,"parent_record_offset":2340,"ancestor_record_offset":260,"inlinee":21857,"source_line":697,"archive":"_pdb/A/modules/02720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::is_full
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":2720,"record_offset":2644,"parent_record_offset":2612,"ancestor_record_offset":260,"inlinee":7710,"source_line":188,"archive":"_pdb/A/modules/02720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::pop_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":2720,"record_offset":692,"parent_record_offset":668,"ancestor_record_offset":260,"inlinee":24159,"source_line":1636,"archive":"_pdb/A/modules/02720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::ptr
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":2720,"record_offset":596,"parent_record_offset":440,"ancestor_record_offset":260,"inlinee":7712,"source_line":159,"archive":"_pdb/A/modules/02720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::swap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":2720,"record_offset":1908,"parent_record_offset":1868,"ancestor_record_offset":260,"inlinee":55582,"source_line":730,"archive":"_pdb/A/modules/02720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":2720,"record_offset":480,"parent_record_offset":440,"ancestor_record_offset":260,"inlinee":7688,"source_line":200,"archive":"_pdb/A/modules/02720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":2720,"record_offset":512,"parent_record_offset":480,"ancestor_record_offset":260,"inlinee":7689,"source_line":195,"archive":"_pdb/A/modules/02720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":2720,"record_offset":4252,"parent_record_offset":4084,"ancestor_record_offset":260,"inlinee":7713,"source_line":207,"archive":"_pdb/A/modules/02720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2720,"record_offset":2692,"parent_record_offset":2668,"ancestor_record_offset":260,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/02720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2720,"record_offset":632,"parent_record_offset":596,"ancestor_record_offset":260,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/02720.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2720,"record_offset":1484,"parent_record_offset":1432,"ancestor_record_offset":260,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/02720.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2720,"record_offset":1456,"parent_record_offset":1432,"ancestor_record_offset":260,"inlinee":21867,"source_line":905,"archive":"_pdb/A/modules/02720.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":2720,"record_offset":1400,"parent_record_offset":724,"ancestor_record_offset":260,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/02720.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":2720,"record_offset":2272,"parent_record_offset":2212,"ancestor_record_offset":260,"inlinee":5999,"source_line":3064,"archive":"_pdb/A/modules/02720.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":2720,"record_offset":2244,"parent_record_offset":2212,"ancestor_record_offset":260,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/02720.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_add
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":2720,"record_offset":544,"parent_record_offset":512,"ancestor_record_offset":260,"inlinee":7690,"source_line":1639,"archive":"_pdb/A/modules/02720.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":2720,"record_offset":4280,"parent_record_offset":4252,"ancestor_record_offset":260,"inlinee":4907,"source_line":1680,"archive":"_pdb/A/modules/02720.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2720,"record_offset":1312,"parent_record_offset":1288,"ancestor_record_offset":260,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/02720.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$216::div
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2720,"record_offset":1140,"parent_record_offset":1112,"ancestor_record_offset":260,"inlinee":5488,"source_line":502,"archive":"_pdb/A/modules/02720.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2720,"record_offset":1052,"parent_record_offset":1024,"ancestor_record_offset":260,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/02720.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::expect
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2720,"record_offset":2580,"parent_record_offset":2340,"ancestor_record_offset":260,"inlinee":5324,"source_line":893,"archive":"_pdb/A/modules/02720.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2720,"record_offset":2212,"parent_record_offset":1908,"ancestor_record_offset":260,"inlinee":47325,"source_line":940,"archive":"_pdb/A/modules/02720.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$10::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec4.rs
//     evidence={"module":2720,"record_offset":1432,"parent_record_offset":724,"ancestor_record_offset":260,"inlinee":21866,"source_line":975,"archive":"_pdb/A/modules/02720.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$12::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2720,"record_offset":1288,"parent_record_offset":724,"ancestor_record_offset":260,"inlinee":17198,"source_line":1002,"archive":"_pdb/A/modules/02720.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2720,"record_offset":1024,"parent_record_offset":724,"ancestor_record_offset":260,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/02720.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$4::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2720,"record_offset":1112,"parent_record_offset":724,"ancestor_record_offset":260,"inlinee":17306,"source_line":923,"archive":"_pdb/A/modules/02720.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::resolve_hole_overlap::check_for_hole_overlap
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 237
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15E2E
// 首条位置: {"module":2720,"inlinee":89646}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":2720,"record_offset":1400,"parent_record_offset":724,"ancestor_record_offset":724,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/02720.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::vec4::impl$10::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec4.rs
//     evidence={"module":2720,"record_offset":1432,"parent_record_offset":724,"ancestor_record_offset":724,"inlinee":21866,"source_line":975,"archive":"_pdb/A/modules/02720.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$12::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2720,"record_offset":1288,"parent_record_offset":724,"ancestor_record_offset":724,"inlinee":17198,"source_line":1002,"archive":"_pdb/A/modules/02720.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2720,"record_offset":1024,"parent_record_offset":724,"ancestor_record_offset":724,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/02720.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$4::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2720,"record_offset":1112,"parent_record_offset":724,"ancestor_record_offset":724,"inlinee":17306,"source_line":923,"archive":"_pdb/A/modules/02720.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2720,"record_offset":1484,"parent_record_offset":1432,"ancestor_record_offset":724,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/02720.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2720,"record_offset":1456,"parent_record_offset":1432,"ancestor_record_offset":724,"inlinee":21867,"source_line":905,"archive":"_pdb/A/modules/02720.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2720,"record_offset":1312,"parent_record_offset":1288,"ancestor_record_offset":724,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/02720.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$216::div
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2720,"record_offset":1140,"parent_record_offset":1112,"ancestor_record_offset":724,"inlinee":5488,"source_line":502,"archive":"_pdb/A/modules/02720.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2720,"record_offset":1052,"parent_record_offset":1024,"ancestor_record_offset":724,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/02720.jsonl.gz"}
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
// 完整结构及原始类型记录: resolve_hole_overlap.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: resolve_hole_overlap.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: resolve_hole_overlap.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::resolve_hole_overlap::check_for_hole_overlap
// 信息 ID: f4124dc024fd4dc7a2481a6e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::utils::resolve_hole_overlap::resolve_hole_overlap; module=2720, inline_record=724; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::resolve_hole_overlap::resolve_hole_overlap
// 信息 ID: 944fc44311892f3a360c72e0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 7 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0xccb798 -> 0x177e540 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0xccb838 -> 0x177e540 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0xccb8ea -> 0x177e540 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_skirts::add_skirts @ 0xcd7f80 -> 0x177e540 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_skirts::add_skirts @ 0xcd8040 -> 0x177e540 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_skirts::add_skirts @ 0xcd80fe -> 0x177e540 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::wall_constructor::WallConstructor::from_curve @ 0xcdd017 -> 0x177e540 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
