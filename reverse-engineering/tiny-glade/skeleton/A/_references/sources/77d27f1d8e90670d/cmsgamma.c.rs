// Generated from PDB metadata by skeleton/_tools/fill_type_structures.py.
// Version: A
// 类型元数据；原作者源码文本/定义文件见逐项映射；不生成依赖实现。

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: cmsgamma.c.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB forward_declaration_only] _cmsParametricCurvesCollection_st; TPI 0x1719
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "_cmsParametricCurvesCollection_st", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [5920]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] _cmsParametricCurvesCollection_st; TPI 0x1720
// 原始定义文件映射: PDB_UDT_source_line
// size = 184; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 5, "Options": ["None"], "FieldList": 5919, "Name": "_cmsParametricCurvesCollection_st", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 184}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5919, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 0, "Name": "nFunctions"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5919, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5917, "FieldOffset": 4, "Name": "FunctionTypes"}}, "type_references": [{"index": 5917, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 116, "IndexType": 35, "Size": 80, "Name": ""}}, "nested_types": {"ElementType": {"index": 116, "spelling": "Int32", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5919, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5918, "FieldOffset": 84, "Name": "ParameterCount"}}, "type_references": [{"index": 5918, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 117, "IndexType": 35, "Size": 80, "Name": ""}}, "nested_types": {"ElementType": {"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5919, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4953, "FieldOffset": 168, "Name": "Evaluator"}}, "type_references": [{"index": 4953, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4952, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4952, "kind": "LF_PROCEDURE", "status": "PDB_record", "record": {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 65, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 3, "ArgumentList": 4951}}, "nested_types": {}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5919, "order": 4, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5914, "FieldOffset": 176, "Name": "Next"}}, "type_references": [{"index": 5914, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 5913, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 5913, "kind": "LF_STRUCTURE", "spelling": "_cmsParametricCurvesCollection_st", "status": "PDB_record_name"}}}]}
// [PDB UDT 源码行] {"ipi_index": 94527, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lcms2-sys-4.0.1/vendor/src/cmsgamma.c", "line": 45, "module": null, "source_string_id": 94526}
// === 完整 TPI 结构关系引用记录；原文不转换成猜测的 Rust ABI ===
// [TPI 0x1355] {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 65, "Modifiers": ["None", "Const"]}}
// [TPI 0x1356] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4949, "Attrs": 65548}}
// [TPI 0x1357] {"Kind": "LF_ARGLIST", "ArgList": {"ArgIndices": [116, 4950, 65]}}
// [TPI 0x1358] {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 65, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 3, "ArgumentList": 4951}}
// [TPI 0x1359] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4952, "Attrs": 65548}}
// [TPI 0x1719] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "_cmsParametricCurvesCollection_st", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x171a] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 5913, "Attrs": 65548}}
// [TPI 0x171d] {"Kind": "LF_ARRAY", "Array": {"ElementType": 116, "IndexType": 35, "Size": 80, "Name": ""}}
// [TPI 0x171e] {"Kind": "LF_ARRAY", "Array": {"ElementType": 117, "IndexType": 35, "Size": 80, "Name": ""}}
// [TPI 0x171f] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 0, "Name": "nFunctions"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5917, "FieldOffset": 4, "Name": "FunctionTypes"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5918, "FieldOffset": 84, "Name": "ParameterCount"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4953, "FieldOffset": 168, "Name": "Evaluator"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5914, "FieldOffset": 176, "Name": "Next"}}]}
// [TPI 0x1720] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 5, "Options": ["None"], "FieldList": 5919, "Name": "_cmsParametricCurvesCollection_st", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 184}}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: cmsgamma.c.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===
