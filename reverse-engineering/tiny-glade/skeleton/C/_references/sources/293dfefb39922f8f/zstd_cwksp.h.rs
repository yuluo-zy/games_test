// Generated from PDB metadata by skeleton/_tools/fill_type_structures.py.
// Version: C
// 类型元数据；原作者源码文本/定义文件见逐项映射；不生成依赖实现。

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: zstd_cwksp.h.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB forward_declaration_only] ZSTD_cwksp; TPI 0x123f
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "ZSTD_cwksp", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [4722]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] <unnamed-tag>; TPI 0x126e
// 原始定义文件映射: PDB_UDT_source_line
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 3, "Options": ["None"], "FieldList": 4717, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4717, "order": 0, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 0, "Name": "ZSTD_cwksp_alloc_objects"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4717, "order": 1, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1, "Name": "ZSTD_cwksp_alloc_buffers"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4717, "order": 2, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 2, "Name": "ZSTD_cwksp_alloc_aligned"}}, "type_references": []}
// [PDB UDT 源码行] {"ipi_index": 101720, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/compress/zstd_cwksp.h", "line": 45, "module": null, "source_string_id": 101719}
// [PDB UDT 源码行] {"ipi_index": 131720, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/dictBuilder/../compress/zstd_cwksp.h", "line": 45, "module": null, "source_string_id": 131719}
// ------------------------------------------------------------------
// [PDB PDB_definition_record] <unnamed-tag>; TPI 0x1270
// 原始定义文件映射: PDB_UDT_source_line
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 2, "Options": ["None"], "FieldList": 4719, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4719, "order": 0, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 0, "Name": "ZSTD_cwksp_dynamic_alloc"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4719, "order": 1, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1, "Name": "ZSTD_cwksp_static_alloc"}}, "type_references": []}
// [PDB UDT 源码行] {"ipi_index": 101721, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/compress/zstd_cwksp.h", "line": 56, "module": null, "source_string_id": 101719}
// [PDB UDT 源码行] {"ipi_index": 131721, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/dictBuilder/../compress/zstd_cwksp.h", "line": 56, "module": null, "source_string_id": 131719}
// ------------------------------------------------------------------
// [PDB PDB_definition_record] ZSTD_cwksp; TPI 0x1272
// 原始定义文件映射: PDB_UDT_source_line
// size = 64; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 10, "Options": ["None"], "FieldList": 4721, "Name": "ZSTD_cwksp", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 64}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4721, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 0, "Name": "workspace"}}, "type_references": [{"index": 1539, "spelling": "Void / CodeView pointer mode 6", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4721, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 8, "Name": "workspaceEnd"}}, "type_references": [{"index": 1539, "spelling": "Void / CodeView pointer mode 6", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4721, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 16, "Name": "objectEnd"}}, "type_references": [{"index": 1539, "spelling": "Void / CodeView pointer mode 6", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4721, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 24, "Name": "tableEnd"}}, "type_references": [{"index": 1539, "spelling": "Void / CodeView pointer mode 6", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4721, "order": 4, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 32, "Name": "tableValidEnd"}}, "type_references": [{"index": 1539, "spelling": "Void / CodeView pointer mode 6", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4721, "order": 5, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 40, "Name": "allocStart"}}, "type_references": [{"index": 1539, "spelling": "Void / CodeView pointer mode 6", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4721, "order": 6, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 32, "FieldOffset": 48, "Name": "allocFailed"}}, "type_references": [{"index": 32, "spelling": "UnsignedCharacter", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4721, "order": 7, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 116, "FieldOffset": 52, "Name": "workspaceOversizedDuration"}}, "type_references": [{"index": 116, "spelling": "Int32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4721, "order": 8, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4718, "FieldOffset": 56, "Name": "phase"}}, "type_references": [{"index": 4718, "kind": "LF_ENUM", "spelling": "<unnamed-tag>", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4721, "order": 9, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4720, "FieldOffset": 60, "Name": "isStatic"}}, "type_references": [{"index": 4720, "kind": "LF_ENUM", "spelling": "<unnamed-tag>", "status": "PDB_record_name"}]}
// [PDB UDT 源码行] {"ipi_index": 101722, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/compress/zstd_cwksp.h", "line": 145, "module": null, "source_string_id": 101719}
// [PDB UDT 源码行] {"ipi_index": 131722, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/dictBuilder/../compress/zstd_cwksp.h", "line": 145, "module": null, "source_string_id": 131719}
// === 完整 TPI 结构关系引用记录；原文不转换成猜测的 Rust ABI ===
// [TPI 0x123f] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "ZSTD_cwksp", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x126d] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 0, "Name": "ZSTD_cwksp_alloc_objects"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1, "Name": "ZSTD_cwksp_alloc_buffers"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 2, "Name": "ZSTD_cwksp_alloc_aligned"}}]}
// [TPI 0x126e] {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 3, "Options": ["None"], "FieldList": 4717, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// [TPI 0x126f] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 0, "Name": "ZSTD_cwksp_dynamic_alloc"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1, "Name": "ZSTD_cwksp_static_alloc"}}]}
// [TPI 0x1270] {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 2, "Options": ["None"], "FieldList": 4719, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// [TPI 0x1271] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 0, "Name": "workspace"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 8, "Name": "workspaceEnd"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 16, "Name": "objectEnd"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 24, "Name": "tableEnd"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 32, "Name": "tableValidEnd"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 40, "Name": "allocStart"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 32, "FieldOffset": 48, "Name": "allocFailed"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 116, "FieldOffset": 52, "Name": "workspaceOversizedDuration"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4718, "FieldOffset": 56, "Name": "phase"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4720, "FieldOffset": 60, "Name": "isStatic"}}]}
// [TPI 0x1272] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 10, "Options": ["None"], "FieldList": 4721, "Name": "ZSTD_cwksp", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 64}}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: zstd_cwksp.h.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===
