// Generated from PDB metadata by skeleton/_tools/fill_type_structures.py.
// Version: B
// 类型元数据；原作者源码文本/定义文件见逐项映射；不生成依赖实现。

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: cmsxform.c.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB forward_declaration_only] _cmsTransformCollection_st; TPI 0x10b9
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "_cmsTransformCollection_st", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [4292]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] _cmsTransformCollection_st; TPI 0x10c4
// 原始定义文件映射: PDB_UDT_source_line
// size = 24; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None"], "FieldList": 4291, "Name": "_cmsTransformCollection_st", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 24}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4291, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4290, "FieldOffset": 0, "Name": "Factory"}}, "type_references": [{"index": 4290, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4289, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4289, "kind": "LF_PROCEDURE", "status": "PDB_record", "record": {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 116, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 7, "ArgumentList": 4288}}, "nested_types": {}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4291, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 116, "FieldOffset": 8, "Name": "OldXform"}}, "type_references": [{"index": 116, "spelling": "Int32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4291, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4282, "FieldOffset": 16, "Name": "Next"}}, "type_references": [{"index": 4282, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4281, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4281, "kind": "LF_STRUCTURE", "spelling": "_cmsTransformCollection_st", "status": "PDB_record_name"}}}]}
// [PDB UDT 源码行] {"ipi_index": 74876, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lcms2-sys-4.0.1/vendor/src/cmsxform.c", "line": 614, "module": null, "source_string_id": 74875}
// === 完整 TPI 结构关系引用记录；原文不转换成猜测的 Rust ABI ===
// [TPI 0x1002] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "_cmsContext_struct", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1003] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4098, "Attrs": 65548}}
// [TPI 0x1013] {"Kind": "LF_ARGLIST", "ArgList": {"ArgIndices": [4099, 1539]}}
// [TPI 0x1014] {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 3, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 2, "ArgumentList": 4115}}
// [TPI 0x1015] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4116, "Attrs": 65548}}
// [TPI 0x101c] {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 3, "Modifiers": ["None", "Const"]}}
// [TPI 0x101d] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4124, "Attrs": 65548}}
// [TPI 0x1031] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 1539, "Attrs": 65548}}
// [TPI 0x1039] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "_cmstransform_struct", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x103a] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4153, "Attrs": 65548}}
// [TPI 0x103d] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "cmsStride", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x103e] {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 4157, "Modifiers": ["None", "Const"]}}
// [TPI 0x103f] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4158, "Attrs": 65548}}
// [TPI 0x1040] {"Kind": "LF_ARGLIST", "ArgList": {"ArgIndices": [4154, 4125, 1539, 117, 117, 4159]}}
// [TPI 0x1041] {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 3, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 6, "ArgumentList": 4160}}
// [TPI 0x1042] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4161, "Attrs": 65548}}
// [TPI 0x104a] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "_cmsPipeline_struct", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x104b] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4170, "Attrs": 65548}}
// [TPI 0x10b9] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "_cmsTransformCollection_st", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x10ba] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4281, "Attrs": 65548}}
// [TPI 0x10bd] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4162, "Attrs": 65548}}
// [TPI 0x10be] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4117, "Attrs": 65548}}
// [TPI 0x10bf] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4171, "Attrs": 65548}}
// [TPI 0x10c0] {"Kind": "LF_ARGLIST", "ArgList": {"ArgIndices": [4285, 4145, 4286, 4287, 1653, 1653, 1653]}}
// [TPI 0x10c1] {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 116, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 7, "ArgumentList": 4288}}
// [TPI 0x10c2] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4289, "Attrs": 65548}}
// [TPI 0x10c3] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4290, "FieldOffset": 0, "Name": "Factory"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 116, "FieldOffset": 8, "Name": "OldXform"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4282, "FieldOffset": 16, "Name": "Next"}}]}
// [TPI 0x10c4] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None"], "FieldList": 4291, "Name": "_cmsTransformCollection_st", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 24}}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: cmsxform.c.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===
