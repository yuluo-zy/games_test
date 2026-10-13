// Generated from PDB metadata by skeleton/_tools/fill_type_structures.py.
// Version: C
// 类型元数据；原作者源码文本/定义文件见逐项映射；不生成依赖实现。

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: corecrt_startup.h.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB PDB_definition_record] _crt_app_type; TPI 0x26bc
// 原始定义文件映射: PDB_UDT_source_line
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 3, "Options": ["None", "HasUniqueName"], "FieldList": 9915, "Name": "_crt_app_type", "UniqueName": ".?AW4_crt_app_type@@", "UnderlyingType": 116}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 9915, "order": 0, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 0, "Name": "_crt_unknown_app"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 9915, "order": 1, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1, "Name": "_crt_console_app"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 9915, "order": 2, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 2, "Name": "_crt_gui_app"}}, "type_references": []}
// [PDB UDT 源码行] {"ipi_index": 148554, "original_path": "D:\\a\\_work\\1\\s\\src\\ExternalAPIs\\WindowsSDKInc\\c\\Include\\10.0.22621.0\\ucrt\\corecrt_startup.h", "line": 47, "module": null, "source_string_id": 148553}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] _onexit_table_t; TPI 0x275b
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "_onexit_table_t", "UniqueName": ".?AU_onexit_table_t@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [10080]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] _onexit_table_t; TPI 0x2760
// 原始定义文件映射: PDB_UDT_source_line
// size = 24; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None", "HasUniqueName"], "FieldList": 10079, "Name": "_onexit_table_t", "UniqueName": ".?AU_onexit_table_t@@", "DerivationList": 0, "VTableShape": 0, "Size": 24}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 10079, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 9950, "FieldOffset": 0, "Name": "_first"}}, "type_references": [{"index": 9950, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 9602, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 9602, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4097, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4097, "kind": "LF_PROCEDURE", "status": "PDB_record", "record": {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 3, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 0, "ArgumentList": 4096}}, "nested_types": {}}}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 10079, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 9950, "FieldOffset": 8, "Name": "_last"}}, "type_references": [{"index": 9950, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 9602, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 9602, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4097, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4097, "kind": "LF_PROCEDURE", "status": "PDB_record", "record": {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 3, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 0, "ArgumentList": 4096}}, "nested_types": {}}}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 10079, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 9950, "FieldOffset": 16, "Name": "_end"}}, "type_references": [{"index": 9950, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 9602, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 9602, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4097, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4097, "kind": "LF_PROCEDURE", "status": "PDB_record", "record": {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 3, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 0, "ArgumentList": 4096}}, "nested_types": {}}}}}}]}
// [PDB UDT 源码行] {"ipi_index": 148704, "original_path": "D:\\a\\_work\\1\\s\\src\\ExternalAPIs\\WindowsSDKInc\\c\\Include\\10.0.22621.0\\ucrt\\corecrt_startup.h", "line": 137, "module": null, "source_string_id": 148553}
// === 完整 TPI 结构关系引用记录；原文不转换成猜测的 Rust ABI ===
// [TPI 0x1000] {"Kind": "LF_ARGLIST", "ArgList": {"ArgIndices": []}}
// [TPI 0x1001] {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 3, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 0, "ArgumentList": 4096}}
// [TPI 0x2582] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4097, "Attrs": 65548}}
// [TPI 0x26bb] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 0, "Name": "_crt_unknown_app"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1, "Name": "_crt_console_app"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 2, "Name": "_crt_gui_app"}}]}
// [TPI 0x26bc] {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 3, "Options": ["None", "HasUniqueName"], "FieldList": 9915, "Name": "_crt_app_type", "UniqueName": ".?AW4_crt_app_type@@", "UnderlyingType": 116}}
// [TPI 0x26de] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 9602, "Attrs": 65548}}
// [TPI 0x275b] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference", "HasUniqueName"], "FieldList": 0, "Name": "_onexit_table_t", "UniqueName": ".?AU_onexit_table_t@@", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x275f] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 9950, "FieldOffset": 0, "Name": "_first"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 9950, "FieldOffset": 8, "Name": "_last"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 9950, "FieldOffset": 16, "Name": "_end"}}]}
// [TPI 0x2760] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None", "HasUniqueName"], "FieldList": 10079, "Name": "_onexit_table_t", "UniqueName": ".?AU_onexit_table_t@@", "DerivationList": 0, "VTableShape": 0, "Size": 24}}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: corecrt_startup.h.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===
