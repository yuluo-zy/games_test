// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: rhapsody2/render-std/rhapsody-post/src/blur_pyramid.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: a69c382f06525e6c7fd09db75aafcdd9a7197c98d4bd07da44b1b799c3efe969
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: blur_pyramid.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 628, 1873

// 归属: dependency_reuse_metadata_only
// Repository: PounceLight/rhapsody2
// Revision: 48efa87114d22e484592a0e9efe866faafa6a876
// Source kind: repository_source

// ------------------------------------------------------------------
// 函数符号: rhapsody_post::blur_pyramid::render_bidirectional_blur_pyramid
// 记录形式: compiled_procedure; instances=1
// 源码行: 9
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":628,"record_offset":4676,"rva":13298528,"va":5382007648}
// 编译实例 RVAs: 0xcaeb60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// ------------------------------------------------------------------
// 函数符号: rhapsody_post::blur_pyramid::render_blur_pyramid
// 记录形式: compiled_procedure; instances=1
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":628,"record_offset":2964,"rva":13297200,"va":5382006320}
// 编译实例 RVAs: 0xcae630
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// ------------------------------------------------------------------
// 函数符号: rhapsody_post::blur_pyramid::render_blur_pyramid::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 21
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9A33
// 首条位置: {"module":628,"inlinee":39475}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// ------------------------------------------------------------------
// 函数符号: rhapsody_post::blur_pyramid::render_blur_pyramid::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 52
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15E53
// 首条位置: {"module":1873,"inlinee":89683}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// ------------------------------------------------------------------
// 函数符号: rhapsody_post::blur_pyramid::render_rev_blur_pyramid
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 84
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9A38
// 首条位置: {"module":628,"inlinee":39480}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// ------------------------------------------------------------------
// 函数符号: rhapsody_post::blur_pyramid::render_rev_blur_pyramid::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 88
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9A3A
// 首条位置: {"module":628,"inlinee":39482}
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
// IPI 0x9A32:
// ---
// Kind:            LF_STRING_ID
// StringId:
//   Id:              0
//   String:          'rhapsody_post::blur_pyramid::render_blur_pyramid'
// ...
// IPI 0x9A33:
// ---
// Kind:            LF_FUNC_ID
// FuncId:
//   ParentScope:     39474
//   FunctionType:    4097
//   Name:            'closure$0'
// ...
// IPI 0x9A37:
// ---
// Kind:            LF_STRING_ID
// StringId:
//   Id:              0
//   String:          'rhapsody_post::blur_pyramid'
// ...
// IPI 0x9A38:
// ---
// Kind:            LF_FUNC_ID
// FuncId:
//   ParentScope:     39479
//   FunctionType:    4097
//   Name:            render_rev_blur_pyramid
// ...
// IPI 0x9A39:
// ---
// Kind:            LF_STRING_ID
// StringId:
//   Id:              0
//   String:          'rhapsody_post::blur_pyramid::render_rev_blur_pyramid'
// ...
// IPI 0x9A3A:
// ---
// Kind:            LF_FUNC_ID
// FuncId:
//   ParentScope:     39481
//   FunctionType:    4097
//   Name:            'closure$0'
// ...
// IPI 0x9A44:
// ---
// Kind:            LF_STRING_ID
// StringId:
//   Id:              0
//   String:          '/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/render-std/rhapsody-post'
// ...
// IPI 0x9A45:
// ---
// Kind:            LF_STRING_ID
// StringId:
//   Id:              0
//   String:          '/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/render-std/rhapsody-post/src/lib.rs/@/rhapsody_post.ec3288a420cf487c-cgu.1'
// ...
// IPI 0x9A46:
// ---
// Kind:            LF_BUILDINFO
// BuildInfo:
//   ArgIndices:      [ 39492, 4268, 39493, 4268, 4268 ]
// ...
// IPI 0x15E52:
// ---
// Kind:            LF_STRING_ID
// StringId:
//   Id:              0
//   String:          'rhapsody_post::blur_pyramid::render_blur_pyramid::closure$0'
// ...
// IPI 0x15E53:
// ---
// Kind:            LF_FUNC_ID
// FuncId:
//   ParentScope:     89682
//   FunctionType:    4097
//   Name:            'closure$0'
// ...
// IPI 0x15E55:
// ---
// Kind:            LF_STRING_ID
// StringId:
//   Id:              0
//   String:          '/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/render-std/rhapsody-post/src/lib.rs/@/rhapsody_post.ec3288a420cf487c-cgu.2'
// ...
// IPI 0x15E56:
// ---
// Kind:            LF_BUILDINFO
// BuildInfo:
//   ArgIndices:      [ 39492, 4268, 89685, 4268, 4268 ]
// ...

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: blur_pyramid.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: blur_pyramid.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===
// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: rhapsody2/render-std/rhapsody-post/src/blur_pyramid.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: blur_pyramid.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// Shader 二进制文件映射 = unknown（没有足够名称/类型关联；不按效果名称强行绑定）。
// [父函数静态调用位置] {"caller_entry_rva":3767344,"caller_aliases":{"alias_group_rva":3767344,"count":1,"display_examples":["rhapsody::rg::compile::compile"],"complete_catalog":"_pdb/C/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":3777034,"position_among_mapped_render_targets":7,"mapped_render_target_count":24,"previous_callsite":3775709,"next_callsite":3777243,"previous_target_files":["crates/tiny-glade/src/render_loop.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/draw_passes/final_post.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":9705472,"caller_aliases":{"alias_group_rva":9705472,"count":1,"display_examples":["country_core::systems::render_loop::vermeer::vermeer_render_from_target"],"complete_catalog":"_pdb/C/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":9705960,"position_among_mapped_render_targets":1,"mapped_render_target_count":1,"previous_callsite":null,"next_callsite":null,"previous_target_files":[],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":13298528,"caller_aliases":{"alias_group_rva":13298528,"count":1,"display_examples":["rhapsody_post::blur_pyramid::render_bidirectional_blur_pyramid"],"complete_catalog":"_pdb/C/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":13298566,"position_among_mapped_render_targets":1,"mapped_render_target_count":1,"previous_callsite":null,"next_callsite":null,"previous_target_files":[],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===
