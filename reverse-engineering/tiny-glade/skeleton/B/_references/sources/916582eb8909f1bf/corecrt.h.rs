// Generated from PDB metadata by skeleton/_tools/fill_type_structures.py.
// Version: B
// 类型元数据；原作者源码文本/定义文件见逐项映射；不生成依赖实现。

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: corecrt.h.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB forward_declaration_only] __crt_locale_pointers; TPI 0x1c2b
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "__crt_locale_pointers", "UniqueName": ".?AU__crt_locale_pointers@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [7218]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] __crt_locale_pointers; TPI 0x1c32
// 原始定义文件映射: PDB_UDT_source_line
// size = 16; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 2, "Options": ["None", "HasUniqueName"], "FieldList": 7217, "Name": "__crt_locale_pointers", "UniqueName": ".?AU__crt_locale_pointers@@", "DerivationList": 0, "VTableShape": 0, "Size": 16}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7217, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7214, "FieldOffset": 0, "Name": "locinfo"}}, "type_references": [{"index": 7214, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 7213, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 7213, "kind": "LF_STRUCTURE", "spelling": "__crt_locale_data", "status": "PDB_record_name"}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7217, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7216, "FieldOffset": 8, "Name": "mbcinfo"}}, "type_references": [{"index": 7216, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 7215, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 7215, "kind": "LF_STRUCTURE", "spelling": "__crt_multibyte_data", "status": "PDB_record_name"}}}]}
// [PDB UDT 源码行] {"ipi_index": 125202, "original_path": "d:\\a01\\_work\\5\\s\\src\\ExternalAPIs\\UnifiedCRT\\inc\\corecrt.h", "line": 619, "module": null, "source_string_id": 125201}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] __crt_locale_data_public; TPI 0x1c35
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "__crt_locale_data_public", "UniqueName": ".?AU__crt_locale_data_public@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [7224]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] __crt_locale_data_public; TPI 0x1c38
// 原始定义文件映射: PDB_UDT_source_line
// size = 16; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None", "HasUniqueName"], "FieldList": 7223, "Name": "__crt_locale_data_public", "UniqueName": ".?AU__crt_locale_data_public@@", "DerivationList": 0, "VTableShape": 0, "Size": 16}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7223, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4134, "FieldOffset": 0, "Name": "_locale_pctype"}}, "type_references": [{"index": 4134, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4133, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4133, "kind": "LF_MODIFIER", "status": "PDB_record", "record": {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 33, "Modifiers": ["None", "Const"]}}, "nested_types": {"ModifiedType": {"index": 33, "spelling": "UInt16Short", "status": "CodeView_simple_type; not Rust source spelling"}}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7223, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 116, "FieldOffset": 8, "Name": "_locale_mb_cur_max"}}, "type_references": [{"index": 116, "spelling": "Int32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7223, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 12, "Name": "_locale_lc_codepage"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [PDB UDT 源码行] {"ipi_index": 125203, "original_path": "d:\\a01\\_work\\5\\s\\src\\ExternalAPIs\\UnifiedCRT\\inc\\corecrt.h", "line": 612, "module": null, "source_string_id": 125201}
// === 完整 TPI 结构关系引用记录；原文不转换成猜测的 Rust ABI ===
// [TPI 0x1025] {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 33, "Modifiers": ["None", "Const"]}}
// [TPI 0x1026] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4133, "Attrs": 65548}}
// [TPI 0x1c2b] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "__crt_locale_pointers", "UniqueName": ".?AU__crt_locale_pointers@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1c2d] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "__crt_locale_data", "UniqueName": ".?AU__crt_locale_data@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1c2e] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 7213, "Attrs": 65548}}
// [TPI 0x1c2f] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "__crt_multibyte_data", "UniqueName": ".?AU__crt_multibyte_data@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1c30] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 7215, "Attrs": 65548}}
// [TPI 0x1c31] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7214, "FieldOffset": 0, "Name": "locinfo"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7216, "FieldOffset": 8, "Name": "mbcinfo"}}]}
// [TPI 0x1c32] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 2, "Options": ["None", "HasUniqueName"], "FieldList": 7217, "Name": "__crt_locale_pointers", "UniqueName": ".?AU__crt_locale_pointers@@", "DerivationList": 0, "VTableShape": 0, "Size": 16}}
// [TPI 0x1c35] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "__crt_locale_data_public", "UniqueName": ".?AU__crt_locale_data_public@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1c37] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4134, "FieldOffset": 0, "Name": "_locale_pctype"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 116, "FieldOffset": 8, "Name": "_locale_mb_cur_max"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 12, "Name": "_locale_lc_codepage"}}]}
// [TPI 0x1c38] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None", "HasUniqueName"], "FieldList": 7223, "Name": "__crt_locale_data_public", "UniqueName": ".?AU__crt_locale_data_public@@", "DerivationList": 0, "VTableShape": 0, "Size": 16}}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: corecrt.h.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===
