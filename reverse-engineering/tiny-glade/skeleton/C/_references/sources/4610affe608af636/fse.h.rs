// Generated from PDB metadata by skeleton/_tools/fill_type_structures.py.
// Version: C
// 类型元数据；原作者源码文本/定义文件见逐项映射；不生成依赖实现。

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: fse.h.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB PDB_definition_record] <unnamed-tag>; TPI 0x12e3
// 原始定义文件映射: PDB_UDT_source_line
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 3, "Options": ["None"], "FieldList": 4834, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4834, "order": 0, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 0, "Name": "FSE_repeat_none"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4834, "order": 1, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1, "Name": "FSE_repeat_check"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4834, "order": 2, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 2, "Name": "FSE_repeat_valid"}}, "type_references": []}
// [PDB UDT 源码行] {"ipi_index": 101761, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/compress/../common/fse.h", "line": 364, "module": null, "source_string_id": 101760}
// [PDB UDT 源码行] {"ipi_index": 131756, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/dictBuilder/../common/fse.h", "line": 364, "module": null, "source_string_id": 131755}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] FSE_CState_t; TPI 0x1a35
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "FSE_CState_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [6714]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] FSE_CState_t; TPI 0x1a3a
// 原始定义文件映射: PDB_UDT_source_line
// size = 32; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 4, "Options": ["None"], "FieldList": 6713, "Name": "FSE_CState_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 32}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6713, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 19, "FieldOffset": 0, "Name": "value"}}, "type_references": [{"index": 19, "spelling": "Int64Quad", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6713, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4125, "FieldOffset": 8, "Name": "stateTable"}}, "type_references": [{"index": 4125, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4124, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4124, "kind": "LF_MODIFIER", "status": "PDB_record", "record": {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 3, "Modifiers": ["None", "Const"]}}, "nested_types": {"ModifiedType": {"index": 3, "spelling": "Void", "status": "CodeView_simple_type; not Rust source spelling"}}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6713, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4125, "FieldOffset": 16, "Name": "symbolTT"}}, "type_references": [{"index": 4125, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4124, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4124, "kind": "LF_MODIFIER", "status": "PDB_record", "record": {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 3, "Modifiers": ["None", "Const"]}}, "nested_types": {"ModifiedType": {"index": 3, "spelling": "Void", "status": "CodeView_simple_type; not Rust source spelling"}}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6713, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 24, "Name": "stateLog"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [PDB UDT 源码行] {"ipi_index": 131397, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/compress/../common/fse.h", "line": 377, "module": null, "source_string_id": 101760}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] FSE_symbolCompressionTransform; TPI 0x1a4f
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "FSE_symbolCompressionTransform", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [6738]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] FSE_symbolCompressionTransform; TPI 0x1a52
// 原始定义文件映射: PDB_UDT_source_line
// size = 8; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 2, "Options": ["None"], "FieldList": 6737, "Name": "FSE_symbolCompressionTransform", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 8}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6737, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 116, "FieldOffset": 0, "Name": "deltaFindState"}}, "type_references": [{"index": 116, "spelling": "Int32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6737, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 4, "Name": "deltaNbBits"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [PDB UDT 源码行] {"ipi_index": 131412, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/compress/../common/fse.h", "line": 509, "module": null, "source_string_id": 101760}
// === 完整 TPI 结构关系引用记录；原文不转换成猜测的 Rust ABI ===
// [TPI 0x101c] {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 3, "Modifiers": ["None", "Const"]}}
// [TPI 0x101d] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4124, "Attrs": 65548}}
// [TPI 0x12e2] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 0, "Name": "FSE_repeat_none"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1, "Name": "FSE_repeat_check"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 2, "Name": "FSE_repeat_valid"}}]}
// [TPI 0x12e3] {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 3, "Options": ["None"], "FieldList": 4834, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// [TPI 0x1a35] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "FSE_CState_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1a39] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 19, "FieldOffset": 0, "Name": "value"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4125, "FieldOffset": 8, "Name": "stateTable"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4125, "FieldOffset": 16, "Name": "symbolTT"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 24, "Name": "stateLog"}}]}
// [TPI 0x1a3a] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 4, "Options": ["None"], "FieldList": 6713, "Name": "FSE_CState_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 32}}
// [TPI 0x1a4f] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "FSE_symbolCompressionTransform", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1a51] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 116, "FieldOffset": 0, "Name": "deltaFindState"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 4, "Name": "deltaNbBits"}}]}
// [TPI 0x1a52] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 2, "Options": ["None"], "FieldList": 6737, "Name": "FSE_symbolCompressionTransform", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 8}}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: fse.h.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===
