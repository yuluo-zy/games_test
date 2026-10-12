// Generated from PDB metadata by skeleton/_tools/fill_type_structures.py.
// Version: B
// 类型元数据；原作者源码文本/定义文件见逐项映射；不生成依赖实现。

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: eh.h.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB forward_declaration_only] _EXCEPTION_POINTERS; TPI 0x2342
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "_EXCEPTION_POINTERS", "UniqueName": ".?AU_EXCEPTION_POINTERS@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [9036]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] _EXCEPTION_POINTERS; TPI 0x234c
// 原始定义文件映射: PDB_UDT_source_line
// size = 16; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 2, "Options": ["None", "HasUniqueName"], "FieldList": 9035, "Name": "_EXCEPTION_POINTERS", "UniqueName": ".?AU_EXCEPTION_POINTERS@@", "DerivationList": 0, "VTableShape": 0, "Size": 16}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 9035, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 9012, "FieldOffset": 0, "Name": "ExceptionRecord"}}, "type_references": [{"index": 9012, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 9011, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 9011, "kind": "LF_STRUCTURE", "spelling": "_EXCEPTION_RECORD", "status": "PDB_record_name"}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 9035, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 8985, "FieldOffset": 8, "Name": "ContextRecord"}}, "type_references": [{"index": 8985, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 8984, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 8984, "kind": "LF_STRUCTURE", "spelling": "_CONTEXT", "status": "PDB_record_name"}}}]}
// [PDB UDT 源码行] {"ipi_index": 134139, "original_path": "d:\\a01\\_work\\5\\s\\src\\ExternalAPIs\\Windows\\10\\sdk\\inc\\winnt.h", "line": 9854, "module": null, "source_string_id": 125204}
// [PDB UDT 源码行] {"ipi_index": 136394, "original_path": "d:\\a01\\_work\\5\\s\\src\\vctools\\crt\\vcruntime\\inc\\eh.h", "line": 31, "module": null, "source_string_id": 136393}
// === 完整 TPI 结构关系引用记录；原文不转换成猜测的 Rust ABI ===
// [TPI 0x2318] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "_CONTEXT", "UniqueName": ".?AU_CONTEXT@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x2319] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 8984, "Attrs": 65548}}
// [TPI 0x2333] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "_EXCEPTION_RECORD", "UniqueName": ".?AU_EXCEPTION_RECORD@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x2334] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 9011, "Attrs": 65548}}
// [TPI 0x2342] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "_EXCEPTION_POINTERS", "UniqueName": ".?AU_EXCEPTION_POINTERS@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x234b] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 9012, "FieldOffset": 0, "Name": "ExceptionRecord"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 8985, "FieldOffset": 8, "Name": "ContextRecord"}}]}
// [TPI 0x234c] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 2, "Options": ["None", "HasUniqueName"], "FieldList": 9035, "Name": "_EXCEPTION_POINTERS", "UniqueName": ".?AU_EXCEPTION_POINTERS@@", "DerivationList": 0, "VTableShape": 0, "Size": 16}}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: eh.h.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===
