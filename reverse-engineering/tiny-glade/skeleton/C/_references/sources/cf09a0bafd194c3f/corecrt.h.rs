// Generated from PDB metadata by skeleton/_tools/fill_type_structures.py.
// Version: C
// 类型元数据；原作者源码文本/定义文件见逐项映射；不生成依赖实现。

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: corecrt.h.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB forward_declaration_only] __crt_locale_pointers; TPI 0x160f
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "__crt_locale_pointers", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [5657]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] __crt_locale_pointers; TPI 0x1619
// 原始定义文件映射: PDB_UDT_source_line
// size = 16; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 2, "Options": ["None"], "FieldList": 5656, "Name": "__crt_locale_pointers", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 16}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5656, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5653, "FieldOffset": 0, "Name": "locinfo"}}, "type_references": [{"index": 5653, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 5652, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 5652, "kind": "LF_STRUCTURE", "spelling": "__crt_locale_data", "status": "PDB_record_name"}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5656, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5655, "FieldOffset": 8, "Name": "mbcinfo"}}, "type_references": [{"index": 5655, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 5654, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 5654, "kind": "LF_STRUCTURE", "spelling": "__crt_multibyte_data", "status": "PDB_record_name"}}}]}
// [PDB UDT 源码行] {"ipi_index": 119127, "original_path": "/home/h3/.cache/cargo-xwin/xwin/sdk/include/ucrt/corecrt.h", "line": 461, "module": null, "source_string_id": 119126}
// === 完整 TPI 结构关系引用记录；原文不转换成猜测的 Rust ABI ===
// [TPI 0x160f] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "__crt_locale_pointers", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1614] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "__crt_locale_data", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1615] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 5652, "Attrs": 65548}}
// [TPI 0x1616] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "__crt_multibyte_data", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1617] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 5654, "Attrs": 65548}}
// [TPI 0x1618] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5653, "FieldOffset": 0, "Name": "locinfo"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5655, "FieldOffset": 8, "Name": "mbcinfo"}}]}
// [TPI 0x1619] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 2, "Options": ["None"], "FieldList": 5656, "Name": "__crt_locale_pointers", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 16}}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: corecrt.h.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===
