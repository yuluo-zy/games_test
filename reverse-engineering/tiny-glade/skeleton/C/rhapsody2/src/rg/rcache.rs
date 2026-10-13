// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: rhapsody2/src/rg/rcache.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 028aef1206202905169879a9fa79b4abadd72ff038071ee1e44b7fdcfe625747
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: rcache.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 800

// 归属: dependency_reuse_metadata_only
// Repository: PounceLight/rhapsody2
// Revision: 48efa87114d22e484592a0e9efe866faafa6a876
// Source kind: repository_source

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::rcache::RenderResourceCache::clear_resources
// 记录形式: compiled_procedure; instances=1
// 源码行: 28
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":800,"record_offset":47756,"rva":15144112,"va":5383853232}
// 编译实例 RVAs: 0xe714b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::rcache::impl$2::default
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 34
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xCAF6
// 首条位置: {"module":800,"inlinee":51958}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::rcache::RenderResourceCache::new
// 记录形式: compiled_procedure; instances=1
// 源码行: 60
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":800,"record_offset":48400,"rva":15144304,"va":5383853424}
// 编译实例 RVAs: 0xe71570
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

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
// === 可用 IPI 函数身份、作用域字符串与构建信息引用图 ===
// IPI 0x10AC:
// ---
// Kind:            LF_STRING_ID
// StringId:
//   Id:              0
//   String:          ''
// ...
// IPI 0xC543:
// ---
// Kind:            LF_STRING_ID
// StringId:
//   Id:              0
//   String:          '/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87'
// ...
// IPI 0xCAF5:
// ---
// Kind:            LF_STRING_ID
// StringId:
//   Id:              0
//   String:          'rhapsody::rg::rcache::impl$2'
// ...
// IPI 0xCAF6:
// ---
// Kind:            LF_FUNC_ID
// FuncId:
//   ParentScope:     51957
//   FunctionType:    4097
//   Name:            default
// ...
// IPI 0xCB9F:
// ---
// Kind:            LF_STRING_ID
// StringId:
//   Id:              0
//   String:          '/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/lib.rs/@/rhapsody.73b8a89e5e911c82-cgu.04'
// ...
// IPI 0xCBA0:
// ---
// Kind:            LF_BUILDINFO
// BuildInfo:
//   ArgIndices:      [ 50499, 4268, 52127, 4268, 4268 ]
// ...

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: rcache.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::rg::rcache::GraphDebugInfo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 8
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::hash_map::HashMap::entry<ref$<str$>,rhapsody::rg::rcache::GraphDebugInfo,rhapsody::hash::FixedHasher>", "constructor": "rhapsody::hash_map::HashMap::entry", "arguments": ["ref$<str$>", "rhapsody::rg::rcache::GraphDebugInfo", "rhapsody::hash::FixedHasher"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::Entry::or_default<ref$<str$>,rhapsody::rg::rcache::GraphDebugInfo,rhapsody::hash::FixedHasher,allocator_api2::stable::alloc::global::Global>", "constructor": "hashbrown::map::Entry::or_default", "arguments": ["ref$<str$>", "rhapsody::rg::rcache::GraphDebugInfo", "rhapsody::hash::FixedHasher", "allocator_api2::stable::alloc::global::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<ref$<str$>,rhapsody::rg::rcache::GraphDebugInfo>", "constructor": "tuple$", "arguments": ["ref$<str$>", "rhapsody::rg::rcache::GraphDebugInfo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTableInner::drop_inner_table<tuple$<ref$<str$>,rhapsody::rg::rcache::GraphDebugInfo>,allocator_api2::stable::alloc::global::Global>", "constructor": "hashbrown::raw::RawTableInner::drop_inner_table", "arguments": ["tuple$<ref$<str$>,rhapsody::rg::rcache::GraphDebugInfo>", "allocator_api2::stable::alloc::global::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<ref$<str$>,rhapsody::rg::rcache::GraphDebugInfo,rhapsody::hash::FixedHasher>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["ref$<str$>", "rhapsody::rg::rcache::GraphDebugInfo", "rhapsody::hash::FixedHasher"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<ref$<str$>,rhapsody::rg::rcache::GraphDebugInfo>,allocator_api2::stable::alloc::global::Global,hashbrown::map::make_hasher::closure_env$0<ref$<str$>,rhapsody::rg::rcache::GraphDebugInfo,rhapsody::hash::FixedHasher> >", "constructor": "hashbrown::raw::impl$8::reserve_rehash::closure_env$1", "arguments": ["tuple$<ref$<str$>,rhapsody::rg::rcache::GraphDebugInfo>", "allocator_api2::stable::alloc::global::Global", "hashbrown::map::make_hasher::closure_env$0<ref$<str$>,rhapsody::rg::rcache::GraphDebugInfo,rhapsody::hash::FixedHasher>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<ref$<str$>,rhapsody::rg::rcache::GraphDebugInfo>,allocator_api2::stable::alloc::global::Global,hashbrown::map::make_hasher::closure_env$0<ref$<str$>,rhapsody::rg::rcache::GraphDebugInfo,rhapsody::hash::FixedHasher> >,tuple$<ptr_mut$<u8> > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<ref$<str$>,rhapsody::rg::rcache::GraphDebugInfo>,allocator_api2::stable::alloc::global::Global,hashbrown::map::make_hasher::closure_env$0<ref$<str$>,rhapsody::rg::rcache::GraphDebugInfo,rhapsody::hash::FixedHasher> >", "tuple$<ptr_mut$<u8> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<ref$<str$>,rhapsody::rg::rcache::GraphDebugInfo>,allocator_api2::stable::alloc::global::Global,hashbrown::map::make_hasher::closure_env$0<ref$<str$>,rhapsody::rg::rcache::GraphDebugInfo,rhapsody::hash::FixedHasher> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<ref$<str$>,rhapsody::rg::rcache::GraphDebugInfo>", "allocator_api2::stable::alloc::global::Global", "hashbrown::map::make_hasher::closure_env$0<ref$<str$>,rhapsody::rg::rcache::GraphDebugInfo,rhapsody::hash::FixedHasher>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::rg::rcache::NodeDebugInfo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_extend::impl$1::spec_extend<rhapsody::rg::rcache::NodeDebugInfo,core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::rg::recorder::RecordedNode>,rhapsody::rg::compile::compile::closure_env$1<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>,tiny_glade::render_loop::compile_main_render_graph::closure_env$0> >,alloc::alloc::Global>", "constructor": "alloc::vec::spec_extend::impl$1::spec_extend", "arguments": ["rhapsody::rg::rcache::NodeDebugInfo", "core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::rg::recorder::RecordedNode>,rhapsody::rg::compile::compile::closure_env$1<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>,tiny_glade::render_loop::compile_main_render_graph::closure_env$0> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::rg::rcache::RenderResourceCache
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 4
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::rg::rcache::RenderResourceCache>", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::rg::rcache::RenderResourceCache"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: rcache.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===
