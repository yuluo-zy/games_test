// Generated from PDB metadata by skeleton/_tools/fill_type_structures.py.
// Version: A
// 类型元数据；原作者源码文本/定义文件见逐项映射；不生成依赖实现。

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: zstd.h.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB forward_declaration_only] ZSTD_customMem; TPI 0x181a
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "ZSTD_customMem", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [6203]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] ZSTD_customMem; TPI 0x183b
// 原始定义文件映射: PDB_UDT_source_line
// size = 24; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None"], "FieldList": 6202, "Name": "ZSTD_customMem", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 24}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6202, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6201, "FieldOffset": 0, "Name": "customAlloc"}}, "type_references": [{"index": 6201, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 6200, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 6200, "kind": "LF_PROCEDURE", "status": "PDB_record", "record": {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 1539, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 2, "ArgumentList": 4147}}, "nested_types": {}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6202, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4601, "FieldOffset": 8, "Name": "customFree"}}, "type_references": [{"index": 4601, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4600, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4600, "kind": "LF_PROCEDURE", "status": "PDB_record", "record": {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 3, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 2, "ArgumentList": 4599}}, "nested_types": {}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6202, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 16, "Name": "opaque"}}, "type_references": [{"index": 1539, "spelling": "Void / CodeView pointer mode 6", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [PDB UDT 源码行] {"ipi_index": 99360, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/compress/../common/../zstd.h", "line": 1587, "module": null, "source_string_id": 99359}
// [PDB UDT 源码行] {"ipi_index": 99756, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/decompress/../common/../zstd.h", "line": 1587, "module": null, "source_string_id": 99741}
// [PDB UDT 源码行] {"ipi_index": 114479, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/common/../zstd.h", "line": 1587, "module": null, "source_string_id": 114478}
// [PDB UDT 源码行] {"ipi_index": 114535, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/dictBuilder/../common/../zstd.h", "line": 1587, "module": null, "source_string_id": 114513}
// [PDB UDT 源码行] {"ipi_index": 114711, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/compress/../zstd.h", "line": 1587, "module": null, "source_string_id": 114703}
// === 完整 TPI 结构关系引用记录；原文不转换成猜测的 Rust ABI ===
// [TPI 0x1033] {"Kind": "LF_ARGLIST", "ArgList": {"ArgIndices": [1539, 35]}}
// [TPI 0x11f7] {"Kind": "LF_ARGLIST", "ArgList": {"ArgIndices": [1539, 1539]}}
// [TPI 0x11f8] {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 3, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 2, "ArgumentList": 4599}}
// [TPI 0x11f9] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4600, "Attrs": 65548}}
// [TPI 0x181a] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "ZSTD_customMem", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1838] {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 1539, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 2, "ArgumentList": 4147}}
// [TPI 0x1839] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 6200, "Attrs": 65548}}
// [TPI 0x183a] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6201, "FieldOffset": 0, "Name": "customAlloc"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4601, "FieldOffset": 8, "Name": "customFree"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 16, "Name": "opaque"}}]}
// [TPI 0x183b] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None"], "FieldList": 6202, "Name": "ZSTD_customMem", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 24}}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: zstd.h.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===
