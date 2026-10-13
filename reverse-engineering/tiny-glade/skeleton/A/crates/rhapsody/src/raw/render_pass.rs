// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/rhapsody/src/raw/render_pass.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 5fccc4f47d498a932dd70d2e84da5d5bf51b03039dd2b3917b086b6dd52eeda4
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: render_pass.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 473, 570, 1329, 1370, 1593, 3007
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use arrayvec; // namespace candidate
//   package identities: [{"name":"arrayvec","source":"PDB_registry_path_only","version":"0.7.4"}]
// use ash; // namespace candidate
//   package identities: [{"name":"ash","source":"PDB_registry_path_only","version":"0.38.0+1.3.281"}]
// use core; // namespace candidate
//   package identities: []
// use rhapsody; // namespace candidate
//   package identities: []
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::render_pass::impl$4::hash
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 10
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF26E
// 首条位置: {"module":1329,"inlinee":62062}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::hash::impls::impl$18::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":1329,"record_offset":47084,"parent_record_offset":47060,"ancestor_record_offset":47060,"inlinee":8459,"source_line":806,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_isize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":1329,"record_offset":47108,"parent_record_offset":47084,"ancestor_record_offset":47060,"inlinee":8460,"source_line":432,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":1329,"record_offset":47132,"parent_record_offset":47108,"ancestor_record_offset":47060,"inlinee":8461,"source_line":395,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":1329,"record_offset":47212,"parent_record_offset":47188,"ancestor_record_offset":47060,"inlinee":8436,"source_line":236,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1329,"record_offset":47156,"parent_record_offset":47132,"ancestor_record_offset":47060,"inlinee":8462,"source_line":2859,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":1329,"record_offset":47188,"parent_record_offset":47132,"ancestor_record_offset":47060,"inlinee":8434,"source_line":132,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::render_pass::impl$11::eq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF7BC
// 首条位置: {"module":1370,"inlinee":63420}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] arrayvec::arrayvec::impl$28::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":1370,"record_offset":35352,"parent_record_offset":35080,"ancestor_record_offset":35080,"inlinee":63422,"source_line":1171,"archive":"_pdb/A/modules/01370.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1370,"record_offset":35132,"parent_record_offset":35080,"ancestor_record_offset":35080,"inlinee":4556,"source_line":2154,"archive":"_pdb/A/modules/01370.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::ArrayVec::as_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":1370,"record_offset":35416,"parent_record_offset":35380,"ancestor_record_offset":35080,"inlinee":23748,"source_line":689,"archive":"_pdb/A/modules/01370.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$3::deref
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":1370,"record_offset":35380,"parent_record_offset":35352,"ancestor_record_offset":35080,"inlinee":23747,"source_line":732,"archive":"_pdb/A/modules/01370.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::as_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec_impl.rs
//     evidence={"module":1370,"record_offset":35440,"parent_record_offset":35416,"ancestor_record_offset":35080,"inlinee":23749,"source_line":17,"archive":"_pdb/A/modules/01370.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::impl$266::eq
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/enums.rs
//     evidence={"module":1370,"record_offset":35280,"parent_record_offset":35224,"ancestor_record_offset":35080,"inlinee":22479,"source_line":36,"archive":"_pdb/A/modules/01370.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::impl$275::eq
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/enums.rs
//     evidence={"module":1370,"record_offset":35308,"parent_record_offset":35224,"ancestor_record_offset":35080,"inlinee":22481,"source_line":55,"archive":"_pdb/A/modules/01370.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::impl$527::eq
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/enums.rs
//     evidence={"module":1370,"record_offset":35252,"parent_record_offset":35224,"ancestor_record_offset":35080,"inlinee":8319,"source_line":649,"archive":"_pdb/A/modules/01370.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1370,"record_offset":35592,"parent_record_offset":35472,"ancestor_record_offset":35080,"inlinee":4576,"source_line":263,"archive":"_pdb/A/modules/01370.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1370,"record_offset":35556,"parent_record_offset":35532,"ancestor_record_offset":35080,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/01370.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1370,"record_offset":35532,"parent_record_offset":35508,"ancestor_record_offset":35080,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/01370.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1370,"record_offset":35508,"parent_record_offset":35472,"ancestor_record_offset":35080,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/01370.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$15::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1370,"record_offset":35164,"parent_record_offset":35132,"ancestor_record_offset":35080,"inlinee":4558,"source_line":2174,"archive":"_pdb/A/modules/01370.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$4::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1370,"record_offset":35472,"parent_record_offset":35380,"ancestor_record_offset":35080,"inlinee":4539,"source_line":58,"archive":"_pdb/A/modules/01370.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":1370,"record_offset":35200,"parent_record_offset":35164,"ancestor_record_offset":35080,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/01370.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::impl$130::eq
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":1370,"record_offset":35224,"parent_record_offset":35200,"ancestor_record_offset":35080,"inlinee":22477,"source_line":670,"archive":"_pdb/A/modules/01370.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$24::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":1370,"record_offset":35732,"parent_record_offset":35616,"ancestor_record_offset":35080,"inlinee":8321,"source_line":49,"archive":"_pdb/A/modules/01370.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::render_pass::impl$8::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6901
// 首条位置: {"module":473,"inlinee":26881}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] arrayvec::arrayvec::impl$26::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":473,"record_offset":4988,"parent_record_offset":4944,"ancestor_record_offset":4944,"inlinee":26883,"source_line":1141,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$5::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":473,"record_offset":5512,"parent_record_offset":4944,"ancestor_record_offset":4944,"inlinee":4512,"source_line":2008,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::ArrayVec::extend_from_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":473,"record_offset":5104,"parent_record_offset":5076,"ancestor_record_offset":4944,"inlinee":26876,"source_line":1068,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$22::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":473,"record_offset":5432,"parent_record_offset":5408,"ancestor_record_offset":4944,"inlinee":26886,"source_line":1031,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$23::extend
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":473,"record_offset":5076,"parent_record_offset":5044,"ancestor_record_offset":4944,"inlinee":26875,"source_line":1046,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$24::extend_from_iter::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":473,"record_offset":5456,"parent_record_offset":5432,"ancestor_record_offset":4944,"inlinee":26888,"source_line":1082,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$25::from_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":473,"record_offset":5044,"parent_record_offset":5016,"ancestor_record_offset":4944,"inlinee":26873,"source_line":1131,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::raw_ptr_add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":473,"record_offset":5352,"parent_record_offset":5104,"ancestor_record_offset":4944,"inlinee":26878,"source_line":1115,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::cloned::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\cloned.rs
//     evidence={"module":473,"record_offset":5156,"parent_record_offset":5104,"ancestor_record_offset":4944,"inlinee":4183,"source_line":40,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":473,"record_offset":5016,"parent_record_offset":4988,"ancestor_record_offset":4944,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::cloned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":473,"record_offset":5264,"parent_record_offset":5156,"ancestor_record_offset":4944,"inlinee":8101,"source_line":1889,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":473,"record_offset":5408,"parent_record_offset":5104,"ancestor_record_offset":4944,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":473,"record_offset":5376,"parent_record_offset":5352,"ancestor_record_offset":4944,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":473,"record_offset":5296,"parent_record_offset":5104,"ancestor_record_offset":4944,"inlinee":7069,"source_line":1486,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":473,"record_offset":5228,"parent_record_offset":5184,"ancestor_record_offset":4944,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":473,"record_offset":5320,"parent_record_offset":5296,"ancestor_record_offset":4944,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":473,"record_offset":5184,"parent_record_offset":5156,"ancestor_record_offset":4944,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::render_pass::impl$9::hash
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF26C
// 首条位置: {"module":1329,"inlinee":62060}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::impl$50::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1329,"record_offset":45996,"parent_record_offset":45968,"ancestor_record_offset":45968,"inlinee":16342,"source_line":567,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::impl$270::hash
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/enums.rs
//     evidence={"module":1329,"record_offset":46456,"parent_record_offset":46220,"ancestor_record_offset":45968,"inlinee":22461,"source_line":36,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::impl$279::hash
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/enums.rs
//     evidence={"module":1329,"record_offset":46656,"parent_record_offset":46220,"ancestor_record_offset":45968,"inlinee":22463,"source_line":55,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::impl$531::hash
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/enums.rs
//     evidence={"module":1329,"record_offset":46256,"parent_record_offset":46220,"ancestor_record_offset":45968,"inlinee":22459,"source_line":649,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_i32
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":1329,"record_offset":46304,"parent_record_offset":46280,"ancestor_record_offset":45968,"inlinee":8469,"source_line":414,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_isize
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":1329,"record_offset":46048,"parent_record_offset":46024,"ancestor_record_offset":45968,"inlinee":8460,"source_line":432,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u32
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":1329,"record_offset":46328,"parent_record_offset":46304,"ancestor_record_offset":45968,"inlinee":8446,"source_line":377,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":1329,"record_offset":46072,"parent_record_offset":46048,"ancestor_record_offset":45968,"inlinee":8461,"source_line":395,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$16::hash
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":1329,"record_offset":46280,"parent_record_offset":46256,"ancestor_record_offset":45968,"inlinee":8468,"source_line":806,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$18::hash
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":1329,"record_offset":46024,"parent_record_offset":45996,"ancestor_record_offset":45968,"inlinee":8459,"source_line":806,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$23::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":1329,"record_offset":46196,"parent_record_offset":45996,"ancestor_record_offset":45968,"inlinee":8466,"source_line":887,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":1329,"record_offset":46152,"parent_record_offset":46128,"ancestor_record_offset":45968,"inlinee":8436,"source_line":236,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1329,"record_offset":46096,"parent_record_offset":46072,"ancestor_record_offset":45968,"inlinee":8462,"source_line":2859,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::to_ne_bytes
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1329,"record_offset":46352,"parent_record_offset":46328,"ancestor_record_offset":45968,"inlinee":8447,"source_line":2859,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::render_pass::impl$4::hash
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/render_pass.rs
//     evidence={"module":1329,"record_offset":47060,"parent_record_offset":46196,"ancestor_record_offset":45968,"inlinee":62062,"source_line":10,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::impl$132::hash
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":1329,"record_offset":46220,"parent_record_offset":46196,"ancestor_record_offset":45968,"inlinee":22457,"source_line":670,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$22::hash
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":1329,"record_offset":46856,"parent_record_offset":46220,"ancestor_record_offset":45968,"inlinee":22465,"source_line":49,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":1329,"record_offset":46128,"parent_record_offset":46072,"ancestor_record_offset":45968,"inlinee":8434,"source_line":132,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::create_render_pass
// 记录形式: compiled_procedure; instances=1
// 源码行: 40
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":570,"record_offset":63696,"rva":7613040,"va":5376322160}
// 编译实例 RVAs: 0x742a70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: render_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3572729","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376322160,"raw_hex":"e8077cc9ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5376322420,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: render_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3573250","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376322160,"raw_hex":"e8fe79c9ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5376322957,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141177170
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10699806","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376322160,"raw_hex":"e81e44a300","size":5,"soft_fail":false,"target_va":5387022704,"va":5376322893,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14117c700
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10722233","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376322160,"raw_hex":"e8b99ba300","size":5,"soft_fail":false,"target_va":5387044608,"va":5376322370,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26655031","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376322160,"raw_hex":"e837b99601","size":5,"soft_fail":false,"target_va":5402978144,"va":5376323108,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26656305","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376322160,"raw_hex":"e831be9601","size":5,"soft_fail":false,"target_va":5402979344,"va":5376323034,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26657037","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376322160,"raw_hex":"e80dc19601","size":5,"soft_fail":false,"target_va":5402980128,"va":5376323086,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5376322160,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5376322797,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] arrayvec::arrayvec::ArrayVec::len
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":570,"record_offset":64220,"parent_record_offset":63696,"ancestor_record_offset":63696,"inlinee":15311,"source_line":113,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] arrayvec::arrayvec::impl$3::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":570,"record_offset":63968,"parent_record_offset":63696,"ancestor_record_offset":63696,"inlinee":23747,"source_line":732,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] ash::device::Device::create_render_pass
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":570,"record_offset":65408,"parent_record_offset":63696,"ancestor_record_offset":63696,"inlinee":31021,"source_line":2396,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::RenderPassCreateInfo::subpasses
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":570,"record_offset":65380,"parent_record_offset":63696,"ancestor_record_offset":63696,"inlinee":31020,"source_line":6161,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::impl$330::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":570,"record_offset":65352,"parent_record_offset":63696,"ancestor_record_offset":63696,"inlinee":31018,"source_line":6129,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":570,"record_offset":64136,"parent_record_offset":63696,"ancestor_record_offset":63696,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":570,"record_offset":63888,"parent_record_offset":63696,"ancestor_record_offset":63696,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":570,"record_offset":65324,"parent_record_offset":63696,"ancestor_record_offset":63696,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":570,"record_offset":63916,"parent_record_offset":63696,"ancestor_record_offset":63696,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":570,"record_offset":65724,"parent_record_offset":63696,"ancestor_record_offset":63696,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":570,"record_offset":66140,"parent_record_offset":63696,"ancestor_record_offset":63696,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":570,"record_offset":64052,"parent_record_offset":63696,"ancestor_record_offset":63696,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::ManagedResources::create_render_pass
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/render_pass.rs
//     evidence={"module":570,"record_offset":65524,"parent_record_offset":63696,"ancestor_record_offset":63696,"inlinee":30446,"source_line":31,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":570,"record_offset":64664,"parent_record_offset":64640,"ancestor_record_offset":63696,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":570,"record_offset":64688,"parent_record_offset":64664,"ancestor_record_offset":63696,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":570,"record_offset":65888,"parent_record_offset":65864,"ancestor_record_offset":63696,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":570,"record_offset":64640,"parent_record_offset":64536,"ancestor_record_offset":63696,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":570,"record_offset":65864,"parent_record_offset":65772,"ancestor_record_offset":63696,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":570,"record_offset":65800,"parent_record_offset":65772,"ancestor_record_offset":63696,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":570,"record_offset":64536,"parent_record_offset":64496,"ancestor_record_offset":63696,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":570,"record_offset":64496,"parent_record_offset":64456,"ancestor_record_offset":63696,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":570,"record_offset":64760,"parent_record_offset":64496,"ancestor_record_offset":63696,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":570,"record_offset":65772,"parent_record_offset":65748,"ancestor_record_offset":63696,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":570,"record_offset":64824,"parent_record_offset":64800,"ancestor_record_offset":63696,"inlinee":5431,"source_line":3007,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":570,"record_offset":64416,"parent_record_offset":64368,"ancestor_record_offset":63696,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":570,"record_offset":64456,"parent_record_offset":64416,"ancestor_record_offset":63696,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":570,"record_offset":64160,"parent_record_offset":64136,"ancestor_record_offset":63696,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":570,"record_offset":65156,"parent_record_offset":65128,"ancestor_record_offset":63696,"inlinee":5435,"source_line":3020,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":570,"record_offset":65220,"parent_record_offset":65156,"ancestor_record_offset":63696,"inlinee":5440,"source_line":18,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":570,"record_offset":64800,"parent_record_offset":64368,"ancestor_record_offset":63696,"inlinee":5430,"source_line":25,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":570,"record_offset":64328,"parent_record_offset":64288,"ancestor_record_offset":63696,"inlinee":7317,"source_line":32,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":570,"record_offset":64368,"parent_record_offset":64328,"ancestor_record_offset":63696,"inlinee":7319,"source_line":52,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::ArrayVec::as_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":570,"record_offset":63992,"parent_record_offset":63968,"ancestor_record_offset":63696,"inlinee":23748,"source_line":689,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::ArrayVec::clear
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":570,"record_offset":66240,"parent_record_offset":66216,"ancestor_record_offset":63696,"inlinee":11795,"source_line":253,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$0::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":570,"record_offset":66216,"parent_record_offset":66192,"ancestor_record_offset":63696,"inlinee":11793,"source_line":49,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$2::set_len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":570,"record_offset":66312,"parent_record_offset":66288,"ancestor_record_offset":63696,"inlinee":10360,"source_line":715,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::as_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec_impl.rs
//     evidence={"module":570,"record_offset":64016,"parent_record_offset":63992,"ancestor_record_offset":63696,"inlinee":23749,"source_line":17,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::clear
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec_impl.rs
//     evidence={"module":570,"record_offset":66264,"parent_record_offset":66240,"ancestor_record_offset":63696,"inlinee":11796,"source_line":72,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::truncate
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec_impl.rs
//     evidence={"module":570,"record_offset":66288,"parent_record_offset":66264,"ancestor_record_offset":63696,"inlinee":10364,"source_line":76,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::assume_init_on_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":570,"record_offset":65436,"parent_record_offset":65408,"ancestor_record_offset":63696,"inlinee":11125,"source_line":24,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":570,"record_offset":65460,"parent_record_offset":65436,"ancestor_record_offset":63696,"inlinee":11115,"source_line":11,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result_with_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":570,"record_offset":65484,"parent_record_offset":65460,"ancestor_record_offset":63696,"inlinee":11116,"source_line":16,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":570,"record_offset":64584,"parent_record_offset":64536,"ancestor_record_offset":63696,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":570,"record_offset":64608,"parent_record_offset":64584,"ancestor_record_offset":63696,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$60::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":570,"record_offset":65064,"parent_record_offset":64968,"ancestor_record_offset":63696,"inlinee":5261,"source_line":1553,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":570,"record_offset":64872,"parent_record_offset":64848,"ancestor_record_offset":63696,"inlinee":4667,"source_line":125,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":570,"record_offset":65100,"parent_record_offset":64896,"ancestor_record_offset":63696,"inlinee":4670,"source_line":89,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$39::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":570,"record_offset":65004,"parent_record_offset":64968,"ancestor_record_offset":63696,"inlinee":5988,"source_line":207,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":570,"record_offset":64968,"parent_record_offset":64936,"ancestor_record_offset":63696,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":570,"record_offset":64936,"parent_record_offset":64896,"ancestor_record_offset":63696,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":570,"record_offset":64184,"parent_record_offset":64160,"ancestor_record_offset":63696,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":570,"record_offset":64896,"parent_record_offset":64872,"ancestor_record_offset":63696,"inlinee":5097,"source_line":2580,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":570,"record_offset":64848,"parent_record_offset":64824,"ancestor_record_offset":63696,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":570,"record_offset":65128,"parent_record_offset":65100,"ancestor_record_offset":63696,"inlinee":5224,"source_line":814,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":570,"record_offset":65828,"parent_record_offset":65800,"ancestor_record_offset":63696,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":570,"record_offset":65028,"parent_record_offset":65004,"ancestor_record_offset":63696,"inlinee":5989,"source_line":520,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":570,"record_offset":65560,"parent_record_offset":65524,"ancestor_record_offset":63696,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":570,"record_offset":65748,"parent_record_offset":65724,"ancestor_record_offset":63696,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":570,"record_offset":64100,"parent_record_offset":64076,"ancestor_record_offset":63696,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":570,"record_offset":64716,"parent_record_offset":64688,"ancestor_record_offset":63696,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":570,"record_offset":65188,"parent_record_offset":65156,"ancestor_record_offset":63696,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":570,"record_offset":64076,"parent_record_offset":64052,"ancestor_record_offset":63696,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::render_pass::impl$1::create_render_pass::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/render_pass.rs
//     evidence={"module":570,"record_offset":65592,"parent_record_offset":65560,"ancestor_record_offset":63696,"inlinee":30432,"source_line":42,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::resources::impl$4::create
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":570,"record_offset":65656,"parent_record_offset":65524,"ancestor_record_offset":63696,"inlinee":11133,"source_line":297,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::resources::impl$9::allocate
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":570,"record_offset":65688,"parent_record_offset":65656,"ancestor_record_offset":63696,"inlinee":31025,"source_line":398,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::render_pass::impl$1::create_render_pass::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/render_pass.rs
//     evidence={"module":570,"record_offset":63940,"parent_record_offset":63916,"ancestor_record_offset":63696,"inlinee":31016,"source_line":69,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::TextureSampleCount::to_vk
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":570,"record_offset":65628,"parent_record_offset":65524,"ancestor_record_offset":63696,"inlinee":31023,"source_line":61,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::render_pass::impl$1::create_render_pass::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 60
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x111C0
// 首条位置: {"module":1593,"inlinee":70080}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::render_pass::impl$1::create_render_pass::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 69
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7928
// 首条位置: {"module":570,"inlinee":31016}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
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
// 完整结构及原始类型记录: render_pass.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::render_pass::RenderPass
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 4
// [PDB 泛型关系，非字段关系] {"type": "tuple$<rhapsody::raw::render_pass::RenderPassDesc,rhapsody::raw::render_pass::RenderPass>", "constructor": "tuple$", "arguments": ["rhapsody::raw::render_pass::RenderPassDesc", "rhapsody::raw::render_pass::RenderPass"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTableInner::drop_inner_table<tuple$<rhapsody::raw::render_pass::RenderPassDesc,rhapsody::raw::render_pass::RenderPass>,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTableInner::drop_inner_table", "arguments": ["tuple$<rhapsody::raw::render_pass::RenderPassDesc,rhapsody::raw::render_pass::RenderPass>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tuple$<rhapsody::raw::render_pass::RenderPassDesc,rhapsody::raw::render_pass::RenderPass> >", "constructor": "core::ptr::drop_in_place", "arguments": ["tuple$<rhapsody::raw::render_pass::RenderPassDesc,rhapsody::raw::render_pass::RenderPass>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<rhapsody::raw::render_pass::RenderPassDesc,rhapsody::raw::render_pass::RenderPass,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["rhapsody::raw::render_pass::RenderPassDesc", "rhapsody::raw::render_pass::RenderPass", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<rhapsody::raw::render_pass::RenderPassDesc,rhapsody::raw::render_pass::RenderPass>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<rhapsody::raw::render_pass::RenderPassDesc,rhapsody::raw::render_pass::RenderPass,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<rhapsody::raw::render_pass::RenderPassDesc,rhapsody::raw::render_pass::RenderPass>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<rhapsody::raw::render_pass::RenderPassDesc,rhapsody::raw::render_pass::RenderPass,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::rustc_entry<rhapsody::raw::render_pass::RenderPassDesc,rhapsody::raw::render_pass::RenderPass,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::rustc_entry", "arguments": ["rhapsody::raw::render_pass::RenderPassDesc", "rhapsody::raw::render_pass::RenderPass", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::render_pass::RenderPassDesc
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 5
// [PDB 泛型关系，非字段关系] {"type": "tuple$<rhapsody::raw::render_pass::RenderPassDesc,rhapsody::raw::render_pass::RenderPass>", "constructor": "tuple$", "arguments": ["rhapsody::raw::render_pass::RenderPassDesc", "rhapsody::raw::render_pass::RenderPass"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTableInner::drop_inner_table<tuple$<rhapsody::raw::render_pass::RenderPassDesc,rhapsody::raw::render_pass::RenderPass>,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTableInner::drop_inner_table", "arguments": ["tuple$<rhapsody::raw::render_pass::RenderPassDesc,rhapsody::raw::render_pass::RenderPass>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tuple$<rhapsody::raw::render_pass::RenderPassDesc,rhapsody::raw::render_pass::RenderPass> >", "constructor": "core::ptr::drop_in_place", "arguments": ["tuple$<rhapsody::raw::render_pass::RenderPassDesc,rhapsody::raw::render_pass::RenderPass>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<rhapsody::raw::render_pass::RenderPassDesc,rhapsody::raw::render_pass::RenderPass,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["rhapsody::raw::render_pass::RenderPassDesc", "rhapsody::raw::render_pass::RenderPass", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<rhapsody::raw::render_pass::RenderPassDesc,rhapsody::raw::render_pass::RenderPass>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<rhapsody::raw::render_pass::RenderPassDesc,rhapsody::raw::render_pass::RenderPass,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<rhapsody::raw::render_pass::RenderPassDesc,rhapsody::raw::render_pass::RenderPass>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<rhapsody::raw::render_pass::RenderPassDesc,rhapsody::raw::render_pass::RenderPass,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<rhapsody::raw::render_pass::RenderPassDesc>", "constructor": "ref$", "arguments": ["rhapsody::raw::render_pass::RenderPassDesc"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<rhapsody::raw::render_pass::RenderPassDesc> >", "constructor": "core::hash::BuildHasher::hash_one", "arguments": ["std::hash::random::RandomState", "ref$<rhapsody::raw::render_pass::RenderPassDesc>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::rustc_entry<rhapsody::raw::render_pass::RenderPassDesc,rhapsody::raw::render_pass::RenderPass,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::rustc_entry", "arguments": ["rhapsody::raw::render_pass::RenderPassDesc", "rhapsody::raw::render_pass::RenderPass", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: render_pass.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: render_pass.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::create_render_pass
// 信息 ID: 69dccc6dcd22b4ac86d78777
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::pass_builder::draw_stream::RasterPass::render @ 0x6616a0 -> 0x742a70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::render_pass::impl$11::eq
// 信息 ID: f09bed981fcc59b74e3ce143
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] hashbrown::map::HashMap::rustc_entry<rhapsody::raw::render_pass::RenderPassDesc,rhapsody::raw::render_pass::RenderPass,std::hash::random::RandomState,alloc::alloc::Global>; module=1370, inline_record=35080; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::render_pass::impl$1::create_render_pass::closure$0
// 信息 ID: 5acd1322eee85458962bd155
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::render_pass::impl$1::create_render_pass", "captures": "unknown"}
// [闭包环境类型引用] rhapsody::raw::render_pass::impl$1::create_render_pass::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<ash::vk::definitions::AttachmentDescription,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::raw::shader::RenderPassAttachmentDesc>,rhapsody::raw::render_pass::impl$1::create_render_pass::closure_env$0>,core::option::IntoIter<ash::vk::definitions::AttachmentDescription> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<ash::vk::definitions::AttachmentDescription,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::raw::shader::RenderPassAttachmentDesc>,rhapsody::raw::render_pass::impl$1::create_render_pass::closure_env$0>,core::option::IntoIter<ash::vk::definitions::AttachmentDescription> > >; module=1593, inline_record=35624; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::render_pass::impl$1::create_render_pass::closure$1
// 信息 ID: 5ed240642e92362584318dd2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "rhapsody::raw::render_pass::impl$1::create_render_pass", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create_render_pass; module=570, inline_record=63940; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::render_pass::impl$4::hash
// 信息 ID: 61644f58ac3f6f2f2427b900
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<rhapsody::raw::render_pass::RenderPassDesc> >; module=1329, inline_record=47060; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::render_pass::impl$8::clone
// 信息 ID: 3d1854dc38c6ec65b25c09d5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::pass_builder::draw_stream::RasterPass::render; module=473, inline_record=4944; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::render_pass::impl$9::hash
// 信息 ID: 45bb3801a529a5db1308e182
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<rhapsody::raw::render_pass::RenderPassDesc> >; module=1329, inline_record=45968; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
