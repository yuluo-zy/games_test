// Generated from PDB metadata by skeleton/_tools/fill_type_structures.py.
// Version: C
// 类型元数据；原作者源码文本/定义文件见逐项映射；不生成依赖实现。

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: zstd_lazy.c.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB PDB_definition_record] <unnamed-tag>; TPI 0x1adb
// 原始定义文件映射: PDB_UDT_source_line
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 3, "Options": ["None"], "FieldList": 6874, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6874, "order": 0, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 0, "Name": "search_hashChain"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6874, "order": 1, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1, "Name": "search_binaryTree"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6874, "order": 2, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 2, "Name": "search_rowHash"}}, "type_references": []}
// [PDB UDT 源码行] {"ipi_index": 131522, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/compress/zstd_lazy.c", "line": 1443, "module": null, "source_string_id": 131521}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] ZSTD_LazyVTable; TPI 0x1adf
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "ZSTD_LazyVTable", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [6888]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] ZSTD_LazyVTable; TPI 0x1ae8
// 原始定义文件映射: PDB_UDT_source_line
// size = 8; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 1, "Options": ["None"], "FieldList": 6887, "Name": "ZSTD_LazyVTable", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 8}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6887, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6886, "FieldOffset": 0, "Name": "searchMax"}}, "type_references": [{"index": 6886, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 6885, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 6885, "kind": "LF_PROCEDURE", "status": "PDB_record", "record": {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 35, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 4, "ArgumentList": 6884}}, "nested_types": {}}}}]}
// [PDB UDT 源码行] {"ipi_index": 131524, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/compress/zstd_lazy.c", "line": 1332, "module": null, "source_string_id": 131521}
// === 完整 TPI 结构关系引用记录；原文不转换成猜测的 Rust ABI ===
// [TPI 0x128e] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "ZSTD_matchState_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x12ac] {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 32, "Modifiers": ["None", "Const"]}}
// [TPI 0x12ad] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4780, "Attrs": 65548}}
// [TPI 0x1421] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4750, "Attrs": 65548}}
// [TPI 0x1ada] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 0, "Name": "search_hashChain"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1, "Name": "search_binaryTree"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 2, "Name": "search_rowHash"}}]}
// [TPI 0x1adb] {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 3, "Options": ["None"], "FieldList": 6874, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// [TPI 0x1adf] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "ZSTD_LazyVTable", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1ae4] {"Kind": "LF_ARGLIST", "ArgList": {"ArgIndices": [5153, 4781, 4781, 1571]}}
// [TPI 0x1ae5] {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 35, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 4, "ArgumentList": 6884}}
// [TPI 0x1ae6] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 6885, "Attrs": 65548}}
// [TPI 0x1ae7] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6886, "FieldOffset": 0, "Name": "searchMax"}}]}
// [TPI 0x1ae8] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 1, "Options": ["None"], "FieldList": 6887, "Name": "ZSTD_LazyVTable", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 8}}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: zstd_lazy.c.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===
