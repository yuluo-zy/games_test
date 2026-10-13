// Generated from PDB metadata by skeleton/_tools/fill_type_structures.py.
// Version: A
// 类型元数据；原作者源码文本/定义文件见逐项映射；不生成依赖实现。

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: stdlib.h.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB forward_declaration_only] _ldiv_t; TPI 0x1143
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "_ldiv_t", "UniqueName": ".?AU_ldiv_t@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [4421]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] _ldiv_t; TPI 0x1145
// 原始定义文件映射: PDB_UDT_source_line
// size = 8; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 2, "Options": ["None", "HasUniqueName"], "FieldList": 4420, "Name": "_ldiv_t", "UniqueName": ".?AU_ldiv_t@@", "DerivationList": 0, "VTableShape": 0, "Size": 8}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4420, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 18, "FieldOffset": 0, "Name": "quot"}}, "type_references": [{"index": 18, "spelling": "Int32Long", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4420, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 18, "FieldOffset": 4, "Name": "rem"}}, "type_references": [{"index": 18, "spelling": "Int32Long", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [PDB UDT 源码行] {"ipi_index": 49858, "original_path": "d:\\a01\\_work\\5\\s\\src\\ExternalAPIs\\UnifiedCRT\\inc\\stdlib.h", "line": 281, "module": null, "source_string_id": 49857}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] _lldiv_t; TPI 0x1146
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "_lldiv_t", "UniqueName": ".?AU_lldiv_t@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [4424]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] _lldiv_t; TPI 0x1148
// 原始定义文件映射: PDB_UDT_source_line
// size = 16; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 2, "Options": ["None", "HasUniqueName"], "FieldList": 4423, "Name": "_lldiv_t", "UniqueName": ".?AU_lldiv_t@@", "DerivationList": 0, "VTableShape": 0, "Size": 16}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4423, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 19, "FieldOffset": 0, "Name": "quot"}}, "type_references": [{"index": 19, "spelling": "Int64Quad", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4423, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 19, "FieldOffset": 8, "Name": "rem"}}, "type_references": [{"index": 19, "spelling": "Int64Quad", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [PDB UDT 源码行] {"ipi_index": 49859, "original_path": "d:\\a01\\_work\\5\\s\\src\\ExternalAPIs\\UnifiedCRT\\inc\\stdlib.h", "line": 287, "module": null, "source_string_id": 49857}
// === 完整 TPI 结构关系引用记录；原文不转换成猜测的 Rust ABI ===
// [TPI 0x1143] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "_ldiv_t", "UniqueName": ".?AU_ldiv_t@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1144] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 18, "FieldOffset": 0, "Name": "quot"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 18, "FieldOffset": 4, "Name": "rem"}}]}
// [TPI 0x1145] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 2, "Options": ["None", "HasUniqueName"], "FieldList": 4420, "Name": "_ldiv_t", "UniqueName": ".?AU_ldiv_t@@", "DerivationList": 0, "VTableShape": 0, "Size": 8}}
// [TPI 0x1146] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "_lldiv_t", "UniqueName": ".?AU_lldiv_t@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1147] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 19, "FieldOffset": 0, "Name": "quot"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 19, "FieldOffset": 8, "Name": "rem"}}]}
// [TPI 0x1148] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 2, "Options": ["None", "HasUniqueName"], "FieldList": 4423, "Name": "_lldiv_t", "UniqueName": ".?AU_lldiv_t@@", "DerivationList": 0, "VTableShape": 0, "Size": 16}}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: stdlib.h.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===
