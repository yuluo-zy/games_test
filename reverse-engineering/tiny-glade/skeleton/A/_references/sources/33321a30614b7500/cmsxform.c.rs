// Generated from PDB metadata by skeleton/_tools/fill_type_structures.py.
// Version: A
// 类型元数据；原作者源码文本/定义文件见逐项映射；不生成依赖实现。

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: cmsxform.c.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB forward_declaration_only] _cmsTransformCollection_st; TPI 0x144b
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "_cmsTransformCollection_st", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [5206]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] _cmsTransformCollection_st; TPI 0x1456
// 原始定义文件映射: PDB_UDT_source_line
// size = 24; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None"], "FieldList": 5205, "Name": "_cmsTransformCollection_st", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 24}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5205, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5204, "FieldOffset": 0, "Name": "Factory"}}, "type_references": [{"index": 5204, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 5203, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 5203, "kind": "LF_PROCEDURE", "status": "PDB_record", "record": {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 116, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 7, "ArgumentList": 5202}}, "nested_types": {}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5205, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 116, "FieldOffset": 8, "Name": "OldXform"}}, "type_references": [{"index": 116, "spelling": "Int32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5205, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5196, "FieldOffset": 16, "Name": "Next"}}, "type_references": [{"index": 5196, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 5195, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 5195, "kind": "LF_STRUCTURE", "spelling": "_cmsTransformCollection_st", "status": "PDB_record_name"}}}]}
// [PDB UDT 源码行] {"ipi_index": 66501, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lcms2-sys-4.0.1/vendor/src/cmsxform.c", "line": 614, "module": null, "source_string_id": 66500}
// === 完整 TPI 结构关系引用记录；原文不转换成猜测的 Rust ABI ===
// [TPI 0x1100] {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 3, "Modifiers": ["None", "Const"]}}
// [TPI 0x1101] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4352, "Attrs": 65548}}
// [TPI 0x1209] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 1539, "Attrs": 65548}}
// [TPI 0x1274] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "_cmsContext_struct", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1275] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4724, "Attrs": 65548}}
// [TPI 0x12ba] {"Kind": "LF_ARGLIST", "ArgList": {"ArgIndices": [4725, 1539]}}
// [TPI 0x12bb] {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 3, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 2, "ArgumentList": 4794}}
// [TPI 0x12bc] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4795, "Attrs": 65548}}
// [TPI 0x1376] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "_cmsPipeline_struct", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x13cb] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4982, "Attrs": 65548}}
// [TPI 0x13f9] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "_cmstransform_struct", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x13fa] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 5113, "Attrs": 65548}}
// [TPI 0x13fb] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "cmsStride", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x13fc] {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 5115, "Modifiers": ["None", "Const"]}}
// [TPI 0x13fd] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 5116, "Attrs": 65548}}
// [TPI 0x13fe] {"Kind": "LF_ARGLIST", "ArgList": {"ArgIndices": [5114, 4353, 1539, 117, 117, 5117]}}
// [TPI 0x13ff] {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 3, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 6, "ArgumentList": 5118}}
// [TPI 0x1400] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 5119, "Attrs": 65548}}
// [TPI 0x144b] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "_cmsTransformCollection_st", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x144c] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 5195, "Attrs": 65548}}
// [TPI 0x144f] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 5120, "Attrs": 65548}}
// [TPI 0x1450] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4796, "Attrs": 65548}}
// [TPI 0x1451] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 5067, "Attrs": 65548}}
// [TPI 0x1452] {"Kind": "LF_ARGLIST", "ArgList": {"ArgIndices": [5199, 4617, 5200, 5201, 1653, 1653, 1653]}}
// [TPI 0x1453] {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 116, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 7, "ArgumentList": 5202}}
// [TPI 0x1454] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 5203, "Attrs": 65548}}
// [TPI 0x1455] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5204, "FieldOffset": 0, "Name": "Factory"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 116, "FieldOffset": 8, "Name": "OldXform"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5196, "FieldOffset": 16, "Name": "Next"}}]}
// [TPI 0x1456] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None"], "FieldList": 5205, "Name": "_cmsTransformCollection_st", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 24}}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: cmsxform.c.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===
