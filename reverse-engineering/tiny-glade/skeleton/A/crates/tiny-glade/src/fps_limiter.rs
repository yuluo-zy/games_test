// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/tiny-glade/src/fps_limiter.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: d37ef552b91bb09e5c007ac8412df4088a7ab5c348b067069a035c134ec9a2bf
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: fps_limiter.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 196
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use core; // namespace candidate
//   package identities: []
// use once_cell; // namespace candidate
//   package identities: [{"name":"once_cell","source":"PDB_registry_path_only","version":"1.19.0"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"PDB_registry_path_only","version":"0.14.1"}]
// use spin_sleep; // namespace candidate
//   package identities: [{"name":"spin_sleep","source":"PDB_registry_path_only","version":"1.1.1"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::fps_limiter::FpsLimiter::frame
// 记录形式: compiled_procedure; instances=1
// 源码行: 16
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":196,"record_offset":7896,"rva":3130224,"va":5371839344}
// 编译实例 RVAs: 0x2fc370
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140854520
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: fps_limiter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5603216","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371839344,"raw_hex":"e8907f5500","size":5,"soft_fail":false,"target_va":5377443104,"va":5371839883,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420a6510
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: fps_limiter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31104230","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371839344,"raw_hex":"e8e69cda01","size":5,"soft_fail":false,"target_va":5402944784,"va":5371840549,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144680
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: fps_limiter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31752114","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371839344,"raw_hex":"e8b27fe401","size":5,"soft_fail":false,"target_va":5403592320,"va":5371840201,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144690
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: fps_limiter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31752087","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371839344,"raw_hex":"e8977fe401","size":5,"soft_fail":false,"target_va":5403592336,"va":5371840244,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31138262","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371839344,"raw_hex":"e8d621db01","size":5,"soft_fail":false,"target_va":5402978688,"va":5371840421,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31138428","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371839344,"raw_hex":"e87c22db01","size":5,"soft_fail":false,"target_va":5402978944,"va":5371840511,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1819","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371839344,"raw_hex":"e8e5f8ffff","size":5,"soft_fail":false,"target_va":5371838448,"va":5371840262,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31140486","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371839344,"raw_hex":"e8862adb01","size":5,"soft_fail":false,"target_va":5402980128,"va":5371839637,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31140929","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371839344,"raw_hex":"e8412cdb01","size":5,"soft_fail":false,"target_va":5402980656,"va":5371839722,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] once_cell::imp::OnceCell$LT$T$GT$::initialize
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t31081803","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371839344,"raw_hex":"e84b45da01","size":5,"soft_fail":false,"target_va":5402922000,"va":5371840192,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4517076","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371839344,"raw_hex":"e8d4ec4400","size":5,"soft_fail":false,"target_va":5376356912,"va":5371839831,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4520987","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371839344,"raw_hex":"e81bfc4400","size":5,"soft_fail":false,"target_va":5376360560,"va":5371839568,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4516728","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371839344,"raw_hex":"e878eb4400","size":5,"soft_fail":false,"target_va":5376356144,"va":5371839411,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::fast_local::Key$LT$T$GT$::try_initialize::h3d9350ee38791ca5 (.llvm.8305701337189838128)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t785575","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371839344,"raw_hex":"e8a7fc0b00","size":5,"soft_fail":false,"target_va":5372625168,"va":5371839588,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::sleep
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5552984","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371839344,"raw_hex":"e858bb5400","size":5,"soft_fail":false,"target_va":5377393200,"va":5371840211,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::time::Instant::now
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5603552","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371839344,"raw_hex":"e8e0805500","size":5,"soft_fail":false,"target_va":5377442944,"va":5371839387,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":196,"record_offset":10124,"parent_record_offset":7896,"ancestor_record_offset":7896,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/00196.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":196,"record_offset":10244,"parent_record_offset":7896,"ancestor_record_offset":7896,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/00196.jsonl.gz"}
//   [pdb_inline_direct] core::time::Duration::as_secs_f32
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\time.rs
//     evidence={"module":196,"record_offset":10212,"parent_record_offset":7896,"ancestor_record_offset":7896,"inlinee":12632,"source_line":855,"archive":"_pdb/A/modules/00196.jsonl.gz"}
//   [pdb_inline_direct] core::time::Duration::from_secs_f32
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\time.rs
//     evidence={"module":196,"record_offset":10272,"parent_record_offset":7896,"ancestor_record_offset":7896,"inlinee":12633,"source_line":968,"archive":"_pdb/A/modules/00196.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":196,"record_offset":9056,"parent_record_offset":7896,"ancestor_record_offset":7896,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/00196.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":196,"record_offset":8048,"parent_record_offset":7896,"ancestor_record_offset":7896,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/00196.jsonl.gz"}
//   [pdb_inline_direct] spin_sleep::native_sleep
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"spin_sleep","version":"1.1.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/spin_sleep-1.1.1/src/lib.rs
//     evidence={"module":196,"record_offset":10488,"parent_record_offset":7896,"ancestor_record_offset":7896,"inlinee":12639,"source_line":115,"archive":"_pdb/A/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":196,"record_offset":9476,"parent_record_offset":9448,"ancestor_record_offset":7896,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":196,"record_offset":9528,"parent_record_offset":9504,"ancestor_record_offset":7896,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":196,"record_offset":9504,"parent_record_offset":9476,"ancestor_record_offset":7896,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":196,"record_offset":9416,"parent_record_offset":9384,"ancestor_record_offset":7896,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":196,"record_offset":9448,"parent_record_offset":9416,"ancestor_record_offset":7896,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":196,"record_offset":9620,"parent_record_offset":9448,"ancestor_record_offset":7896,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":196,"record_offset":9732,"parent_record_offset":9708,"ancestor_record_offset":7896,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::impl$46::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\num.rs
//     evidence={"module":196,"record_offset":10424,"parent_record_offset":10308,"ancestor_record_offset":7896,"inlinee":6006,"source_line":76,"archive":"_pdb/A/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::to_bits
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":196,"record_offset":10368,"parent_record_offset":10308,"ancestor_record_offset":7896,"inlinee":12635,"source_line":1110,"archive":"_pdb/A/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::to_bits::rt_f32_to_u32
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":196,"record_offset":10392,"parent_record_offset":10368,"ancestor_record_offset":7896,"inlinee":12637,"source_line":1150,"archive":"_pdb/A/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":196,"record_offset":10180,"parent_record_offset":10124,"ancestor_record_offset":7896,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":196,"record_offset":9552,"parent_record_offset":9528,"ancestor_record_offset":7896,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":196,"record_offset":8920,"parent_record_offset":8828,"ancestor_record_offset":7896,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":196,"record_offset":9308,"parent_record_offset":9280,"ancestor_record_offset":7896,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":196,"record_offset":8636,"parent_record_offset":8176,"ancestor_record_offset":7896,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":196,"record_offset":9356,"parent_record_offset":9176,"ancestor_record_offset":7896,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":196,"record_offset":9004,"parent_record_offset":8976,"ancestor_record_offset":7896,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":196,"record_offset":9684,"parent_record_offset":9384,"ancestor_record_offset":7896,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":196,"record_offset":8412,"parent_record_offset":8388,"ancestor_record_offset":7896,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":196,"record_offset":8388,"parent_record_offset":8364,"ancestor_record_offset":7896,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":196,"record_offset":8596,"parent_record_offset":8564,"ancestor_record_offset":7896,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":196,"record_offset":9576,"parent_record_offset":9552,"ancestor_record_offset":7896,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":196,"record_offset":10084,"parent_record_offset":9136,"ancestor_record_offset":7896,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":196,"record_offset":8864,"parent_record_offset":8828,"ancestor_record_offset":7896,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":196,"record_offset":8888,"parent_record_offset":8864,"ancestor_record_offset":7896,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":196,"record_offset":8364,"parent_record_offset":8340,"ancestor_record_offset":7896,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":196,"record_offset":8692,"parent_record_offset":8668,"ancestor_record_offset":7896,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":196,"record_offset":8340,"parent_record_offset":8232,"ancestor_record_offset":7896,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":196,"record_offset":8828,"parent_record_offset":8796,"ancestor_record_offset":7896,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":196,"record_offset":8076,"parent_record_offset":8048,"ancestor_record_offset":7896,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":196,"record_offset":8204,"parent_record_offset":8176,"ancestor_record_offset":7896,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":196,"record_offset":8176,"parent_record_offset":8148,"ancestor_record_offset":7896,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":196,"record_offset":8124,"parent_record_offset":8100,"ancestor_record_offset":7896,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":196,"record_offset":8148,"parent_record_offset":8124,"ancestor_record_offset":7896,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":196,"record_offset":8100,"parent_record_offset":8076,"ancestor_record_offset":7896,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":196,"record_offset":8668,"parent_record_offset":8148,"ancestor_record_offset":7896,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":196,"record_offset":8796,"parent_record_offset":8764,"ancestor_record_offset":7896,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":196,"record_offset":8952,"parent_record_offset":8796,"ancestor_record_offset":7896,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":196,"record_offset":8764,"parent_record_offset":8740,"ancestor_record_offset":7896,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":196,"record_offset":8740,"parent_record_offset":8048,"ancestor_record_offset":7896,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":196,"record_offset":8976,"parent_record_offset":8952,"ancestor_record_offset":7896,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":196,"record_offset":8232,"parent_record_offset":8204,"ancestor_record_offset":7896,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":196,"record_offset":8480,"parent_record_offset":8232,"ancestor_record_offset":7896,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":196,"record_offset":8452,"parent_record_offset":8232,"ancestor_record_offset":7896,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":196,"record_offset":8284,"parent_record_offset":8232,"ancestor_record_offset":7896,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":196,"record_offset":10668,"parent_record_offset":10644,"ancestor_record_offset":7896,"inlinee":6265,"source_line":1431,"archive":"_pdb/A/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":196,"record_offset":10692,"parent_record_offset":10668,"ancestor_record_offset":7896,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] core::time::Duration::try_from_secs_f32
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\time.rs
//     evidence={"module":196,"record_offset":10308,"parent_record_offset":10272,"ancestor_record_offset":7896,"inlinee":12634,"source_line":1602,"archive":"_pdb/A/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::imp::OnceCell::is_initialized
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"once_cell","version":"1.19.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/once_cell-1.19.0/src/imp_std.rs
//     evidence={"module":196,"record_offset":10644,"parent_record_offset":10620,"ancestor_record_offset":7896,"inlinee":6941,"source_line":49,"archive":"_pdb/A/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::Lazy::force
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"once_cell","version":"1.19.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/once_cell-1.19.0/src/lib.rs
//     evidence={"module":196,"record_offset":10544,"parent_record_offset":10520,"ancestor_record_offset":7896,"inlinee":6935,"source_line":1308,"archive":"_pdb/A/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"once_cell","version":"1.19.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/once_cell-1.19.0/src/lib.rs
//     evidence={"module":196,"record_offset":10620,"parent_record_offset":10592,"ancestor_record_offset":7896,"inlinee":6939,"source_line":963,"archive":"_pdb/A/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get_or_init
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"once_cell","version":"1.19.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/once_cell-1.19.0/src/lib.rs
//     evidence={"module":196,"record_offset":10568,"parent_record_offset":10544,"ancestor_record_offset":7896,"inlinee":6937,"source_line":1115,"archive":"_pdb/A/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get_or_try_init
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"once_cell","version":"1.19.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/once_cell-1.19.0/src/lib.rs
//     evidence={"module":196,"record_offset":10592,"parent_record_offset":10568,"ancestor_record_offset":7896,"inlinee":6938,"source_line":1152,"archive":"_pdb/A/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::impl$12::deref
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"once_cell","version":"1.19.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/once_cell-1.19.0/src/lib.rs
//     evidence={"module":196,"record_offset":10520,"parent_record_offset":10488,"ancestor_record_offset":7896,"inlinee":6933,"source_line":1375,"archive":"_pdb/A/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":196,"record_offset":9096,"parent_record_offset":9056,"ancestor_record_offset":7896,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":196,"record_offset":9220,"parent_record_offset":9176,"ancestor_record_offset":7896,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":196,"record_offset":9384,"parent_record_offset":9176,"ancestor_record_offset":7896,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":196,"record_offset":9656,"parent_record_offset":9384,"ancestor_record_offset":7896,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":196,"record_offset":9248,"parent_record_offset":9220,"ancestor_record_offset":7896,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":196,"record_offset":9280,"parent_record_offset":9248,"ancestor_record_offset":7896,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":196,"record_offset":9176,"parent_record_offset":9136,"ancestor_record_offset":7896,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00196.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":196,"record_offset":9136,"parent_record_offset":9096,"ancestor_record_offset":7896,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00196.jsonl.gz"}
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
// 完整结构及原始类型记录: fps_limiter.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::fps_limiter::FpsLimiter
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 1
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: fps_limiter.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: fps_limiter.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: tiny_glade::fps_limiter::FpsLimiter::frame
// 信息 ID: 9d42f75a7d23c36a2b0a047f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::game_loop::GameLoop::main_loop @ 0x24c2ca -> 0x2fc370 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
