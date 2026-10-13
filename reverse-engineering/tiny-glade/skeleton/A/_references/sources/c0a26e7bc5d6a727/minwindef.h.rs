// Generated from PDB metadata by skeleton/_tools/fill_type_structures.py.
// Version: A
// 类型元数据；原作者源码文本/定义文件见逐项映射；不生成依赖实现。

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: minwindef.h.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB forward_declaration_only] HINSTANCE__; TPI 0x2772
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "HINSTANCE__", "UniqueName": ".?AUHINSTANCE__@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [10105]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] HINSTANCE__; TPI 0x2779
// 原始定义文件映射: PDB_UDT_source_line
// size = 4; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 1, "Options": ["None", "HasUniqueName"], "FieldList": 10104, "Name": "HINSTANCE__", "UniqueName": ".?AUHINSTANCE__@@", "DerivationList": 0, "VTableShape": 0, "Size": 4}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 10104, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 116, "FieldOffset": 0, "Name": "unused"}}, "type_references": [{"index": 116, "spelling": "Int32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [PDB UDT 源码行] {"ipi_index": 123541, "original_path": "d:\\a01\\_work\\5\\s\\src\\ExternalAPIs\\Windows\\10\\sdk\\inc\\minwindef.h", "line": 250, "module": null, "source_string_id": 123540}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] _FILETIME; TPI 0x2783
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "_FILETIME", "UniqueName": ".?AU_FILETIME@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [10122]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] _FILETIME; TPI 0x278a
// 原始定义文件映射: PDB_UDT_source_line
// size = 8; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 2, "Options": ["None", "HasUniqueName"], "FieldList": 10121, "Name": "_FILETIME", "UniqueName": ".?AU_FILETIME@@", "DerivationList": 0, "VTableShape": 0, "Size": 8}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 10121, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 34, "FieldOffset": 0, "Name": "dwLowDateTime"}}, "type_references": [{"index": 34, "spelling": "UInt32Long", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 10121, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 34, "FieldOffset": 4, "Name": "dwHighDateTime"}}, "type_references": [{"index": 34, "spelling": "UInt32Long", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [PDB UDT 源码行] {"ipi_index": 123578, "original_path": "d:\\a01\\_work\\5\\s\\src\\ExternalAPIs\\Windows\\10\\sdk\\inc\\minwindef.h", "line": 271, "module": null, "source_string_id": 123540}
// === 完整 TPI 结构关系引用记录；原文不转换成猜测的 Rust ABI ===
// [TPI 0x2772] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "HINSTANCE__", "UniqueName": ".?AUHINSTANCE__@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x2778] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 116, "FieldOffset": 0, "Name": "unused"}}]}
// [TPI 0x2779] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 1, "Options": ["None", "HasUniqueName"], "FieldList": 10104, "Name": "HINSTANCE__", "UniqueName": ".?AUHINSTANCE__@@", "DerivationList": 0, "VTableShape": 0, "Size": 4}}
// [TPI 0x2783] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "_FILETIME", "UniqueName": ".?AU_FILETIME@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x2789] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 34, "FieldOffset": 0, "Name": "dwLowDateTime"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 34, "FieldOffset": 4, "Name": "dwHighDateTime"}}]}
// [TPI 0x278a] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 2, "Options": ["None", "HasUniqueName"], "FieldList": 10121, "Name": "_FILETIME", "UniqueName": ".?AU_FILETIME@@", "DerivationList": 0, "VTableShape": 0, "Size": 8}}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: minwindef.h.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===
