// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/rhapsody/src/raw/render_pass.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: edf46e2788f83158737326e933d1008cc1dfaa5432d9ad0719d875e4aa6ee2ad
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: render_pass.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 31, 240, 482, 703, 1582, 1613, 1935, 1936
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use arrayvec; // namespace candidate
//   package identities: [{"name":"arrayvec","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.7.4"}]
// use ash; // namespace candidate
//   package identities: [{"name":"ash","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.38.0+1.3.281"}]
// use core; // namespace candidate
//   package identities: []
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"local_or_path","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::render_pass::impl$4::hash
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 12
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11A27
// 首条位置: {"module":1582,"inlinee":72231}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::hash::impls::impl$18::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1582,"record_offset":51540,"parent_record_offset":51516,"ancestor_record_offset":51516,"inlinee":12339,"source_line":811,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_isize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1582,"record_offset":51564,"parent_record_offset":51540,"ancestor_record_offset":51516,"inlinee":10581,"source_line":430,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1582,"record_offset":51588,"parent_record_offset":51564,"ancestor_record_offset":51516,"inlinee":15756,"source_line":393,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1582,"record_offset":51664,"parent_record_offset":51640,"ancestor_record_offset":51516,"inlinee":8969,"source_line":231,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1582,"record_offset":51612,"parent_record_offset":51588,"ancestor_record_offset":51516,"inlinee":15757,"source_line":3479,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1582,"record_offset":51640,"parent_record_offset":51588,"ancestor_record_offset":51516,"inlinee":8977,"source_line":132,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::render_pass::impl$11::eq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 19
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x120ED
// 首条位置: {"module":1613,"inlinee":73965}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] arrayvec::arrayvec::impl$28::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":1613,"record_offset":41756,"parent_record_offset":41704,"ancestor_record_offset":41704,"inlinee":73967,"source_line":1171,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1613,"record_offset":42260,"parent_record_offset":41704,"ancestor_record_offset":41704,"inlinee":5835,"source_line":2186,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::ArrayVec::as_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":1613,"record_offset":41808,"parent_record_offset":41784,"ancestor_record_offset":41704,"inlinee":26718,"source_line":689,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::ArrayVec::len
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":1613,"record_offset":41880,"parent_record_offset":41856,"ancestor_record_offset":41704,"inlinee":14608,"source_line":113,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$2::len
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":1613,"record_offset":41856,"parent_record_offset":41832,"ancestor_record_offset":41704,"inlinee":26720,"source_line":713,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$3::deref
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":1613,"record_offset":41784,"parent_record_offset":41756,"ancestor_record_offset":41704,"inlinee":26717,"source_line":732,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::as_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec_impl.rs
//     evidence={"module":1613,"record_offset":41832,"parent_record_offset":41808,"ancestor_record_offset":41704,"inlinee":26719,"source_line":17,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::impl$266::eq
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/enums.rs
//     evidence={"module":1613,"record_offset":42156,"parent_record_offset":42096,"ancestor_record_offset":41704,"inlinee":33002,"source_line":36,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::impl$275::eq
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/enums.rs
//     evidence={"module":1613,"record_offset":42184,"parent_record_offset":42096,"ancestor_record_offset":41704,"inlinee":33004,"source_line":55,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::impl$527::eq
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/enums.rs
//     evidence={"module":1613,"record_offset":42128,"parent_record_offset":42096,"ancestor_record_offset":41704,"inlinee":8804,"source_line":649,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1613,"record_offset":42072,"parent_record_offset":41952,"ancestor_record_offset":41704,"inlinee":4974,"source_line":261,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1613,"record_offset":42036,"parent_record_offset":42012,"ancestor_record_offset":41704,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1613,"record_offset":42012,"parent_record_offset":41988,"ancestor_record_offset":41704,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1613,"record_offset":41988,"parent_record_offset":41952,"ancestor_record_offset":41704,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1613,"record_offset":41924,"parent_record_offset":41756,"ancestor_record_offset":41704,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$4::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1613,"record_offset":41952,"parent_record_offset":41924,"ancestor_record_offset":41704,"inlinee":5713,"source_line":57,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$10::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\tuple.rs
//     evidence={"module":1613,"record_offset":42292,"parent_record_offset":42260,"ancestor_record_offset":41704,"inlinee":4662,"source_line":30,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::impl$130::eq
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":1613,"record_offset":42096,"parent_record_offset":42072,"ancestor_record_offset":41704,"inlinee":33000,"source_line":681,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$24::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":1613,"record_offset":42212,"parent_record_offset":42096,"ancestor_record_offset":41704,"inlinee":12549,"source_line":49,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::render_pass::impl$8::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 19
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x686C
// 首条位置: {"module":482,"inlinee":26732}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] arrayvec::arrayvec::impl$26::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":482,"record_offset":14608,"parent_record_offset":14576,"ancestor_record_offset":14576,"inlinee":26734,"source_line":1141,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$5::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":482,"record_offset":15244,"parent_record_offset":14576,"ancestor_record_offset":14576,"inlinee":4547,"source_line":2037,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::ArrayVec::as_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":482,"record_offset":14664,"parent_record_offset":14632,"ancestor_record_offset":14576,"inlinee":26718,"source_line":689,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::ArrayVec::extend_from_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":482,"record_offset":14828,"parent_record_offset":14804,"ancestor_record_offset":14576,"inlinee":26725,"source_line":1068,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::ArrayVec::len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":482,"record_offset":14736,"parent_record_offset":14712,"ancestor_record_offset":14576,"inlinee":14608,"source_line":113,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$22::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":482,"record_offset":15164,"parent_record_offset":15140,"ancestor_record_offset":14576,"inlinee":26738,"source_line":1031,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$23::extend
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":482,"record_offset":14804,"parent_record_offset":14776,"ancestor_record_offset":14576,"inlinee":26724,"source_line":1046,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$24::extend_from_iter::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":482,"record_offset":15188,"parent_record_offset":15164,"ancestor_record_offset":14576,"inlinee":26740,"source_line":1082,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$25::from_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":482,"record_offset":14776,"parent_record_offset":14632,"ancestor_record_offset":14576,"inlinee":26722,"source_line":1131,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$2::len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":482,"record_offset":14712,"parent_record_offset":14688,"ancestor_record_offset":14576,"inlinee":26720,"source_line":713,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$3::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":482,"record_offset":14632,"parent_record_offset":14608,"ancestor_record_offset":14576,"inlinee":26717,"source_line":732,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::raw_ptr_add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":482,"record_offset":15084,"parent_record_offset":14828,"ancestor_record_offset":14576,"inlinee":26727,"source_line":1115,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::as_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec_impl.rs
//     evidence={"module":482,"record_offset":14688,"parent_record_offset":14664,"ancestor_record_offset":14576,"inlinee":26719,"source_line":17,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::cloned::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\cloned.rs
//     evidence={"module":482,"record_offset":14872,"parent_record_offset":14828,"ancestor_record_offset":14576,"inlinee":5077,"source_line":40,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::cloned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":482,"record_offset":14968,"parent_record_offset":14872,"ancestor_record_offset":14576,"inlinee":5078,"source_line":1918,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":482,"record_offset":15140,"parent_record_offset":14828,"ancestor_record_offset":14576,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":482,"record_offset":15108,"parent_record_offset":15084,"ancestor_record_offset":14576,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":482,"record_offset":15028,"parent_record_offset":14828,"ancestor_record_offset":14576,"inlinee":5670,"source_line":1505,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":482,"record_offset":14936,"parent_record_offset":14900,"ancestor_record_offset":14576,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":482,"record_offset":15052,"parent_record_offset":15028,"ancestor_record_offset":14576,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":482,"record_offset":14900,"parent_record_offset":14872,"ancestor_record_offset":14576,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::impl$127::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":482,"record_offset":14992,"parent_record_offset":14968,"ancestor_record_offset":14576,"inlinee":26736,"source_line":681,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::render_pass::impl$9::hash
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 19
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11A25
// 首条位置: {"module":1582,"inlinee":72229}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::impl$47::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1582,"record_offset":50472,"parent_record_offset":50444,"ancestor_record_offset":50444,"inlinee":37048,"source_line":567,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::impl$270::hash
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/enums.rs
//     evidence={"module":1582,"record_offset":50924,"parent_record_offset":50692,"ancestor_record_offset":50444,"inlinee":32990,"source_line":36,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::impl$279::hash
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/enums.rs
//     evidence={"module":1582,"record_offset":51120,"parent_record_offset":50692,"ancestor_record_offset":50444,"inlinee":32992,"source_line":55,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::impl$531::hash
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/enums.rs
//     evidence={"module":1582,"record_offset":50728,"parent_record_offset":50692,"ancestor_record_offset":50444,"inlinee":32988,"source_line":649,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_i32
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1582,"record_offset":50776,"parent_record_offset":50752,"ancestor_record_offset":50444,"inlinee":10584,"source_line":412,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_isize
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1582,"record_offset":50524,"parent_record_offset":50500,"ancestor_record_offset":50444,"inlinee":10581,"source_line":430,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u32
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1582,"record_offset":50800,"parent_record_offset":50776,"ancestor_record_offset":50444,"inlinee":8974,"source_line":375,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1582,"record_offset":50548,"parent_record_offset":50524,"ancestor_record_offset":50444,"inlinee":15756,"source_line":393,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$16::hash
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1582,"record_offset":50752,"parent_record_offset":50728,"ancestor_record_offset":50444,"inlinee":15885,"source_line":811,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$18::hash
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1582,"record_offset":50500,"parent_record_offset":50472,"ancestor_record_offset":50444,"inlinee":12339,"source_line":811,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$23::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1582,"record_offset":50668,"parent_record_offset":50472,"ancestor_record_offset":50444,"inlinee":9028,"source_line":892,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1582,"record_offset":50624,"parent_record_offset":50600,"ancestor_record_offset":50444,"inlinee":8969,"source_line":231,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1582,"record_offset":50572,"parent_record_offset":50548,"ancestor_record_offset":50444,"inlinee":15757,"source_line":3479,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::to_ne_bytes
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1582,"record_offset":50824,"parent_record_offset":50800,"ancestor_record_offset":50444,"inlinee":8975,"source_line":3479,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::render_pass::impl$4::hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/render_pass.rs
//     evidence={"module":1582,"record_offset":51516,"parent_record_offset":50668,"ancestor_record_offset":50444,"inlinee":72231,"source_line":12,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::impl$132::hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":1582,"record_offset":50692,"parent_record_offset":50668,"ancestor_record_offset":50444,"inlinee":32986,"source_line":681,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$22::hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":1582,"record_offset":51316,"parent_record_offset":50692,"ancestor_record_offset":50444,"inlinee":32994,"source_line":49,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1582,"record_offset":50600,"parent_record_offset":50548,"ancestor_record_offset":50444,"inlinee":8977,"source_line":132,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::render_pass::impl$13::default
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 41
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1F9F
// 首条位置: {"module":31,"inlinee":8095}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":31,"record_offset":10904,"parent_record_offset":10880,"ancestor_record_offset":10880,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":31,"record_offset":11432,"parent_record_offset":11400,"ancestor_record_offset":10880,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":31,"record_offset":11492,"parent_record_offset":11460,"ancestor_record_offset":10880,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":31,"record_offset":11288,"parent_record_offset":11264,"ancestor_record_offset":10880,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":31,"record_offset":11460,"parent_record_offset":11400,"ancestor_record_offset":10880,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":31,"record_offset":11036,"parent_record_offset":11000,"ancestor_record_offset":10880,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":31,"record_offset":11152,"parent_record_offset":11124,"ancestor_record_offset":10880,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":31,"record_offset":11240,"parent_record_offset":11124,"ancestor_record_offset":10880,"inlinee":7403,"source_line":1581,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":31,"record_offset":11316,"parent_record_offset":11288,"ancestor_record_offset":10880,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":31,"record_offset":11264,"parent_record_offset":11240,"ancestor_record_offset":10880,"inlinee":7404,"source_line":1212,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":31,"record_offset":11344,"parent_record_offset":11288,"ancestor_record_offset":10880,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":31,"record_offset":10952,"parent_record_offset":10928,"ancestor_record_offset":10880,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":31,"record_offset":11200,"parent_record_offset":11176,"ancestor_record_offset":10880,"inlinee":7402,"source_line":83,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":31,"record_offset":11064,"parent_record_offset":11036,"ancestor_record_offset":10880,"inlinee":7399,"source_line":97,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":31,"record_offset":11400,"parent_record_offset":11000,"ancestor_record_offset":10880,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":31,"record_offset":10928,"parent_record_offset":10904,"ancestor_record_offset":10880,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":31,"record_offset":11092,"parent_record_offset":11064,"ancestor_record_offset":10880,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::initialize
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":31,"record_offset":11124,"parent_record_offset":11092,"ancestor_record_offset":10880,"inlinee":7400,"source_line":61,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":31,"record_offset":11000,"parent_record_offset":10976,"ancestor_record_offset":10880,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":31,"record_offset":10976,"parent_record_offset":10952,"ancestor_record_offset":10880,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::create_render_pass
// 记录形式: compiled_procedure; instances=1
// 源码行: 47
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":703,"record_offset":116404,"rva":12566736,"va":5381275856}
// 编译实例 RVAs: 0xbfc0d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: render_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6194636","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381275856,"raw_hex":"e8347aa1ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5381276119,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: render_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6195167","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381275856,"raw_hex":"e82178a1ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5381276666,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418ae9f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13313586","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381275856,"raw_hex":"e83226cb00","size":5,"soft_fail":false,"target_va":5394590192,"va":5381276601,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418b8c50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13355683","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381275856,"raw_hex":"e8a3cacb00","size":5,"soft_fail":false,"target_va":5394631760,"va":5381276072,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30264166","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381275856,"raw_hex":"e866cbcd01","size":5,"soft_fail":false,"target_va":5411540963,"va":5381276792,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30265481","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381275856,"raw_hex":"e889d0cd01","size":5,"soft_fail":false,"target_va":5411542296,"va":5381276810,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30266112","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381275856,"raw_hex":"e800d3cd01","size":5,"soft_fail":false,"target_va":5411542880,"va":5381276763,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*640(%rdi)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5381275856,"raw_hex":"ff9780020000","size":6,"soft_fail":false,"va":5381276488,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] arrayvec::arrayvec::ArrayVec::len
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":703,"record_offset":117044,"parent_record_offset":116404,"ancestor_record_offset":116404,"inlinee":14608,"source_line":113,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] arrayvec::arrayvec::impl$3::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":703,"record_offset":116680,"parent_record_offset":116404,"ancestor_record_offset":116404,"inlinee":26717,"source_line":732,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] ash::device::Device::create_render_pass
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":703,"record_offset":118392,"parent_record_offset":116404,"ancestor_record_offset":116404,"inlinee":39733,"source_line":2396,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::RenderPassCreateInfo::subpasses
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":703,"record_offset":118364,"parent_record_offset":116404,"ancestor_record_offset":116404,"inlinee":39732,"source_line":6161,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::impl$330::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":703,"record_offset":118336,"parent_record_offset":116404,"ancestor_record_offset":116404,"inlinee":39730,"source_line":6129,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::chain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":703,"record_offset":116904,"parent_record_offset":116404,"ancestor_record_offset":116404,"inlinee":4543,"source_line":468,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":703,"record_offset":116960,"parent_record_offset":116404,"ancestor_record_offset":116404,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":703,"record_offset":116600,"parent_record_offset":116404,"ancestor_record_offset":116404,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":703,"record_offset":118308,"parent_record_offset":116404,"ancestor_record_offset":116404,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":703,"record_offset":116628,"parent_record_offset":116404,"ancestor_record_offset":116404,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":703,"record_offset":118708,"parent_record_offset":116404,"ancestor_record_offset":116404,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":703,"record_offset":119140,"parent_record_offset":116404,"ancestor_record_offset":116404,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":703,"record_offset":116820,"parent_record_offset":116404,"ancestor_record_offset":116404,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::ManagedResources::create_render_pass
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/render_pass.rs
//     evidence={"module":703,"record_offset":118508,"parent_record_offset":116404,"ancestor_record_offset":116404,"inlinee":39734,"source_line":31,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":703,"record_offset":117680,"parent_record_offset":117656,"ancestor_record_offset":116404,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":703,"record_offset":117704,"parent_record_offset":117680,"ancestor_record_offset":116404,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":703,"record_offset":118860,"parent_record_offset":118836,"ancestor_record_offset":116404,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":703,"record_offset":117656,"parent_record_offset":117460,"ancestor_record_offset":116404,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":703,"record_offset":118836,"parent_record_offset":118780,"ancestor_record_offset":116404,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":117360,"parent_record_offset":117312,"ancestor_record_offset":116404,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":118808,"parent_record_offset":118780,"ancestor_record_offset":116404,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":118780,"parent_record_offset":118756,"ancestor_record_offset":116404,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":117460,"parent_record_offset":117408,"ancestor_record_offset":116404,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":117408,"parent_record_offset":117360,"ancestor_record_offset":116404,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":118756,"parent_record_offset":118732,"ancestor_record_offset":116404,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":117516,"parent_record_offset":117460,"ancestor_record_offset":116404,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":703,"record_offset":117788,"parent_record_offset":117760,"ancestor_record_offset":116404,"inlinee":4678,"source_line":3552,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":703,"record_offset":117264,"parent_record_offset":117208,"ancestor_record_offset":116404,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":703,"record_offset":117312,"parent_record_offset":117264,"ancestor_record_offset":116404,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":703,"record_offset":116984,"parent_record_offset":116960,"ancestor_record_offset":116404,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":703,"record_offset":118140,"parent_record_offset":118112,"ancestor_record_offset":116404,"inlinee":4708,"source_line":3565,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":703,"record_offset":118204,"parent_record_offset":118140,"ancestor_record_offset":116404,"inlinee":4710,"source_line":18,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":703,"record_offset":117760,"parent_record_offset":117208,"ancestor_record_offset":116404,"inlinee":4677,"source_line":28,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":703,"record_offset":117164,"parent_record_offset":117120,"ancestor_record_offset":116404,"inlinee":4673,"source_line":33,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":703,"record_offset":117208,"parent_record_offset":117164,"ancestor_record_offset":116404,"inlinee":4675,"source_line":52,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::ArrayVec::as_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":703,"record_offset":116704,"parent_record_offset":116680,"ancestor_record_offset":116404,"inlinee":26718,"source_line":689,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::ArrayVec::clear
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":703,"record_offset":119240,"parent_record_offset":119216,"ancestor_record_offset":116404,"inlinee":6205,"source_line":253,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::ArrayVec::len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":703,"record_offset":116776,"parent_record_offset":116752,"ancestor_record_offset":116404,"inlinee":14608,"source_line":113,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$0::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":703,"record_offset":119216,"parent_record_offset":119192,"ancestor_record_offset":116404,"inlinee":6203,"source_line":49,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$2::len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":703,"record_offset":116752,"parent_record_offset":116728,"ancestor_record_offset":116404,"inlinee":26720,"source_line":713,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$2::set_len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":703,"record_offset":119312,"parent_record_offset":119288,"ancestor_record_offset":116404,"inlinee":6210,"source_line":715,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::as_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec_impl.rs
//     evidence={"module":703,"record_offset":116728,"parent_record_offset":116704,"ancestor_record_offset":116404,"inlinee":26719,"source_line":17,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::clear
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec_impl.rs
//     evidence={"module":703,"record_offset":119264,"parent_record_offset":119240,"ancestor_record_offset":116404,"inlinee":6207,"source_line":72,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::truncate
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec_impl.rs
//     evidence={"module":703,"record_offset":119288,"parent_record_offset":119264,"ancestor_record_offset":116404,"inlinee":6208,"source_line":76,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::assume_init_on_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":703,"record_offset":118420,"parent_record_offset":118392,"ancestor_record_offset":116404,"inlinee":15819,"source_line":24,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":703,"record_offset":118444,"parent_record_offset":118420,"ancestor_record_offset":116404,"inlinee":15820,"source_line":11,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result_with_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":703,"record_offset":118468,"parent_record_offset":118444,"ancestor_record_offset":116404,"inlinee":15821,"source_line":16,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":703,"record_offset":117540,"parent_record_offset":117516,"ancestor_record_offset":116404,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":703,"record_offset":117564,"parent_record_offset":117540,"ancestor_record_offset":116404,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$64::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":703,"record_offset":118048,"parent_record_offset":117952,"ancestor_record_offset":116404,"inlinee":5290,"source_line":1824,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::Chain::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":703,"record_offset":116928,"parent_record_offset":116904,"ancestor_record_offset":116404,"inlinee":4545,"source_line":36,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":703,"record_offset":117844,"parent_record_offset":117816,"ancestor_record_offset":116404,"inlinee":4681,"source_line":124,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":703,"record_offset":118084,"parent_record_offset":117872,"ancestor_record_offset":116404,"inlinee":4692,"source_line":88,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$39::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":703,"record_offset":117992,"parent_record_offset":117952,"ancestor_record_offset":116404,"inlinee":5292,"source_line":203,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":703,"record_offset":117952,"parent_record_offset":117916,"ancestor_record_offset":116404,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":703,"record_offset":117916,"parent_record_offset":117872,"ancestor_record_offset":116404,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":703,"record_offset":117008,"parent_record_offset":116984,"ancestor_record_offset":116404,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":703,"record_offset":117872,"parent_record_offset":117844,"ancestor_record_offset":116404,"inlinee":5298,"source_line":2539,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":703,"record_offset":117816,"parent_record_offset":117788,"ancestor_record_offset":116404,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":703,"record_offset":118112,"parent_record_offset":118084,"ancestor_record_offset":116404,"inlinee":4706,"source_line":797,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":703,"record_offset":117588,"parent_record_offset":117564,"ancestor_record_offset":116404,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":703,"record_offset":117612,"parent_record_offset":117588,"ancestor_record_offset":116404,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":703,"record_offset":119036,"parent_record_offset":119008,"ancestor_record_offset":116404,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":703,"record_offset":118016,"parent_record_offset":117992,"ancestor_record_offset":116404,"inlinee":5293,"source_line":560,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":703,"record_offset":118544,"parent_record_offset":118508,"ancestor_record_offset":116404,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":703,"record_offset":118732,"parent_record_offset":118708,"ancestor_record_offset":116404,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":703,"record_offset":116868,"parent_record_offset":116844,"ancestor_record_offset":116404,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":703,"record_offset":118172,"parent_record_offset":118140,"ancestor_record_offset":116404,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":703,"record_offset":116844,"parent_record_offset":116820,"ancestor_record_offset":116404,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::render_pass::impl$1::create_render_pass::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/render_pass.rs
//     evidence={"module":703,"record_offset":118576,"parent_record_offset":118544,"ancestor_record_offset":116404,"inlinee":39736,"source_line":42,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::resources::impl$4::create
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":703,"record_offset":118640,"parent_record_offset":118508,"ancestor_record_offset":116404,"inlinee":15367,"source_line":298,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::resources::impl$9::allocate
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":703,"record_offset":118672,"parent_record_offset":118640,"ancestor_record_offset":116404,"inlinee":39738,"source_line":399,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::render_pass::impl$1::create_render_pass::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/render_pass.rs
//     evidence={"module":703,"record_offset":116652,"parent_record_offset":116628,"ancestor_record_offset":116404,"inlinee":39728,"source_line":76,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::TextureSampleCount::to_vk
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":703,"record_offset":118612,"parent_record_offset":118508,"ancestor_record_offset":116404,"inlinee":39036,"source_line":61,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::render_pass::impl$1::create_render_pass::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 67
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x152D6
// 首条位置: {"module":1935,"inlinee":86742}
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
// 源码行: 76
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9B30
// 首条位置: {"module":703,"inlinee":39728}
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
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<rhapsody::raw::render_pass::RenderPassDesc,rhapsody::raw::render_pass::RenderPass,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["rhapsody::raw::render_pass::RenderPassDesc", "rhapsody::raw::render_pass::RenderPass", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<rhapsody::raw::render_pass::RenderPassDesc,rhapsody::raw::render_pass::RenderPass>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<rhapsody::raw::render_pass::RenderPassDesc,rhapsody::raw::render_pass::RenderPass,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::impl$8::reserve_rehash::closure_env$1", "arguments": ["tuple$<rhapsody::raw::render_pass::RenderPassDesc,rhapsody::raw::render_pass::RenderPass>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<rhapsody::raw::render_pass::RenderPassDesc,rhapsody::raw::render_pass::RenderPass,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<rhapsody::raw::render_pass::RenderPassDesc,rhapsody::raw::render_pass::RenderPass>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<rhapsody::raw::render_pass::RenderPassDesc,rhapsody::raw::render_pass::RenderPass,std::hash::random::RandomState> >,tuple$<ptr_mut$<u8> > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<rhapsody::raw::render_pass::RenderPassDesc,rhapsody::raw::render_pass::RenderPass>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<rhapsody::raw::render_pass::RenderPassDesc,rhapsody::raw::render_pass::RenderPass,std::hash::random::RandomState> >", "tuple$<ptr_mut$<u8> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<rhapsody::raw::render_pass::RenderPassDesc,rhapsody::raw::render_pass::RenderPass>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<rhapsody::raw::render_pass::RenderPassDesc,rhapsody::raw::render_pass::RenderPass,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<rhapsody::raw::render_pass::RenderPassDesc,rhapsody::raw::render_pass::RenderPass>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<rhapsody::raw::render_pass::RenderPassDesc,rhapsody::raw::render_pass::RenderPass,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::rustc_entry<rhapsody::raw::render_pass::RenderPassDesc,rhapsody::raw::render_pass::RenderPass,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::rustc_entry", "arguments": ["rhapsody::raw::render_pass::RenderPassDesc", "rhapsody::raw::render_pass::RenderPass", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::render_pass::RenderPassDesc
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 5
// [PDB 泛型关系，非字段关系] {"type": "tuple$<rhapsody::raw::render_pass::RenderPassDesc,rhapsody::raw::render_pass::RenderPass>", "constructor": "tuple$", "arguments": ["rhapsody::raw::render_pass::RenderPassDesc", "rhapsody::raw::render_pass::RenderPass"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTableInner::drop_inner_table<tuple$<rhapsody::raw::render_pass::RenderPassDesc,rhapsody::raw::render_pass::RenderPass>,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTableInner::drop_inner_table", "arguments": ["tuple$<rhapsody::raw::render_pass::RenderPassDesc,rhapsody::raw::render_pass::RenderPass>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<rhapsody::raw::render_pass::RenderPassDesc,rhapsody::raw::render_pass::RenderPass,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["rhapsody::raw::render_pass::RenderPassDesc", "rhapsody::raw::render_pass::RenderPass", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<rhapsody::raw::render_pass::RenderPassDesc,rhapsody::raw::render_pass::RenderPass>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<rhapsody::raw::render_pass::RenderPassDesc,rhapsody::raw::render_pass::RenderPass,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::impl$8::reserve_rehash::closure_env$1", "arguments": ["tuple$<rhapsody::raw::render_pass::RenderPassDesc,rhapsody::raw::render_pass::RenderPass>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<rhapsody::raw::render_pass::RenderPassDesc,rhapsody::raw::render_pass::RenderPass,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<rhapsody::raw::render_pass::RenderPassDesc,rhapsody::raw::render_pass::RenderPass>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<rhapsody::raw::render_pass::RenderPassDesc,rhapsody::raw::render_pass::RenderPass,std::hash::random::RandomState> >,tuple$<ptr_mut$<u8> > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<rhapsody::raw::render_pass::RenderPassDesc,rhapsody::raw::render_pass::RenderPass>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<rhapsody::raw::render_pass::RenderPassDesc,rhapsody::raw::render_pass::RenderPass,std::hash::random::RandomState> >", "tuple$<ptr_mut$<u8> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)()
// 候选参数类型: []

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::create_render_pass
// 信息 ID: 81169d412de46f6cfcbe3483
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::pass_builder::draw_stream::RasterPass::render @ 0x868815 -> 0xbfc0d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::render_pass::impl$11::eq
// 信息 ID: 5e35365734dcfbb6ebd04a65
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] hashbrown::map::HashMap::rustc_entry<rhapsody::raw::render_pass::RenderPassDesc,rhapsody::raw::render_pass::RenderPass,std::hash::random::RandomState,alloc::alloc::Global>; module=1613, inline_record=41704; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::render_pass::impl$13::default
// 信息 ID: 331a9d8c18c3a8c187e77643
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] std::sys::backtrace::__rust_begin_short_backtrace<tiny_glade::run_game::run_game_thread::closure_env$8,tuple$<> >; module=31, inline_record=10880; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::run_game::run_game_thread; module=240, inline_record=117380; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::render_pass::impl$1::create_render_pass::closure$0
// 信息 ID: fa4b8d28155321757ff3b934
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::render_pass::impl$1::create_render_pass", "captures": "unknown"}
// [闭包环境类型引用] rhapsody::raw::render_pass::impl$1::create_render_pass::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<ash::vk::definitions::AttachmentDescription,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::raw::shader::RenderPassAttachmentDesc>,rhapsody::raw::render_pass::impl$1::create_render_pass::closure_env$0>,core::option::IntoIter<ash::vk::definitions::AttachmentDescription> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<ash::vk::definitions::AttachmentDescription,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::raw::shader::RenderPassAttachmentDesc>,rhapsody::raw::render_pass::impl$1::create_render_pass::closure_env$0>,core::option::IntoIter<ash::vk::definitions::AttachmentDescription> > >; module=1936, inline_record=65120; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::render_pass::impl$1::create_render_pass::closure$1
// 信息 ID: 563892de34f6ace4ada1c003
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "rhapsody::raw::render_pass::impl$1::create_render_pass", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create_render_pass; module=703, inline_record=116652; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::render_pass::impl$4::hash
// 信息 ID: 1c13a3fbb119253a87caca62
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<rhapsody::raw::render_pass::RenderPassDesc> >; module=1582, inline_record=51516; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::render_pass::impl$8::clone
// 信息 ID: 6127b655660bc51dbae41052
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::pass_builder::draw_stream::RasterPass::render; module=482, inline_record=14576; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::render_pass::impl$9::hash
// 信息 ID: b1b62dfb33c513d8def7c3a4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<rhapsody::raw::render_pass::RenderPassDesc> >; module=1582, inline_record=50444; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
