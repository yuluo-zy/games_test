// Generated from PDB metadata by skeleton/_tools/fill_type_structures.py.
// Version: C
// 类型元数据；原作者源码文本/定义文件见逐项映射；不生成依赖实现。

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: cmscnvrt.c.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB forward_declaration_only] _cms_intents_list; TPI 0x17ba
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "_cms_intents_list", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [6081]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] _cms_intents_list; TPI 0x17c1
// 原始定义文件映射: PDB_UDT_source_line
// size = 280; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 4, "Options": ["None"], "FieldList": 6080, "Name": "_cms_intents_list", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 280}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6080, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 0, "Name": "Intent"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6080, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4227, "FieldOffset": 4, "Name": "Description"}}, "type_references": [{"index": 4227, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 112, "IndexType": 35, "Size": 256, "Name": ""}}, "nested_types": {"ElementType": {"index": 112, "spelling": "NarrowCharacter", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6080, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6079, "FieldOffset": 264, "Name": "Link"}}, "type_references": [{"index": 6079, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 6078, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 6078, "kind": "LF_PROCEDURE", "status": "PDB_record", "record": {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 4171, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 7, "ArgumentList": 6077}}, "nested_types": {}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6080, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6075, "FieldOffset": 272, "Name": "Next"}}, "type_references": [{"index": 6075, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 6074, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 6074, "kind": "LF_STRUCTURE", "spelling": "_cms_intents_list", "status": "PDB_record_name"}}}]}
// [PDB UDT 源码行] {"ipi_index": 119569, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lcms2-sys-4.0.1/vendor/src/cmscnvrt.c", "line": 71, "module": null, "source_string_id": 119568}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] GrayOnlyParams; TPI 0x17e0
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "GrayOnlyParams", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [6114]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] GrayOnlyParams; TPI 0x17e2
// 原始定义文件映射: PDB_UDT_source_line
// size = 16; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 2, "Options": ["None"], "FieldList": 6113, "Name": "GrayOnlyParams", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 16}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6113, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4171, "FieldOffset": 0, "Name": "cmyk2cmyk"}}, "type_references": [{"index": 4171, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4170, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4170, "kind": "LF_STRUCTURE", "spelling": "_cmsPipeline_struct", "status": "PDB_record_name"}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6113, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4540, "FieldOffset": 8, "Name": "KTone"}}, "type_references": [{"index": 4540, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4539, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4539, "kind": "LF_STRUCTURE", "spelling": "_cms_curve_struct", "status": "PDB_record_name"}}}]}
// [PDB UDT 源码行] {"ipi_index": 119589, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lcms2-sys-4.0.1/vendor/src/cmscnvrt.c", "line": 691, "module": null, "source_string_id": 119568}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] PreserveKPlaneParams; TPI 0x17e3
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "PreserveKPlaneParams", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [6117]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] PreserveKPlaneParams; TPI 0x17e5
// 原始定义文件映射: PDB_UDT_source_line
// size = 64; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 8, "Options": ["None"], "FieldList": 6116, "Name": "PreserveKPlaneParams", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 64}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6116, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4171, "FieldOffset": 0, "Name": "cmyk2cmyk"}}, "type_references": [{"index": 4171, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4170, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4170, "kind": "LF_STRUCTURE", "spelling": "_cmsPipeline_struct", "status": "PDB_record_name"}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6116, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 8, "Name": "hProofOutput"}}, "type_references": [{"index": 1539, "spelling": "Void / CodeView pointer mode 6", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6116, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 16, "Name": "cmyk2Lab"}}, "type_references": [{"index": 1539, "spelling": "Void / CodeView pointer mode 6", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6116, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4540, "FieldOffset": 24, "Name": "KTone"}}, "type_references": [{"index": 4540, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4539, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4539, "kind": "LF_STRUCTURE", "spelling": "_cms_curve_struct", "status": "PDB_record_name"}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6116, "order": 4, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4171, "FieldOffset": 32, "Name": "LabK2cmyk"}}, "type_references": [{"index": 4171, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4170, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4170, "kind": "LF_STRUCTURE", "spelling": "_cmsPipeline_struct", "status": "PDB_record_name"}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6116, "order": 5, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 65, "FieldOffset": 40, "Name": "MaxError"}}, "type_references": [{"index": 65, "spelling": "Float64", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6116, "order": 6, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 48, "Name": "hRoundTrip"}}, "type_references": [{"index": 1539, "spelling": "Void / CodeView pointer mode 6", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6116, "order": 7, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 65, "FieldOffset": 56, "Name": "MaxTAC"}}, "type_references": [{"index": 65, "spelling": "Float64", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [PDB UDT 源码行] {"ipi_index": 119591, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lcms2-sys-4.0.1/vendor/src/cmscnvrt.c", "line": 841, "module": null, "source_string_id": 119568}
// === 完整 TPI 结构关系引用记录；原文不转换成猜测的 Rust ABI ===
// [TPI 0x1002] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "_cmsContext_struct", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1003] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4098, "Attrs": 65548}}
// [TPI 0x1031] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 1539, "Attrs": 65548}}
// [TPI 0x104a] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "_cmsPipeline_struct", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x104b] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4170, "Attrs": 65548}}
// [TPI 0x1083] {"Kind": "LF_ARRAY", "Array": {"ElementType": 112, "IndexType": 35, "Size": 256, "Name": ""}}
// [TPI 0x11bb] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "_cms_curve_struct", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x11bc] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4539, "Attrs": 65548}}
// [TPI 0x17ba] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "_cms_intents_list", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x17bb] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 6074, "Attrs": 65548}}
// [TPI 0x17bd] {"Kind": "LF_ARGLIST", "ArgList": {"ArgIndices": [4099, 117, 1653, 4145, 1652, 1601, 117]}}
// [TPI 0x17be] {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 4171, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 7, "ArgumentList": 6077}}
// [TPI 0x17bf] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 6078, "Attrs": 65548}}
// [TPI 0x17c0] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 0, "Name": "Intent"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4227, "FieldOffset": 4, "Name": "Description"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6079, "FieldOffset": 264, "Name": "Link"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6075, "FieldOffset": 272, "Name": "Next"}}]}
// [TPI 0x17c1] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 4, "Options": ["None"], "FieldList": 6080, "Name": "_cms_intents_list", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 280}}
// [TPI 0x17e0] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "GrayOnlyParams", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x17e1] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4171, "FieldOffset": 0, "Name": "cmyk2cmyk"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4540, "FieldOffset": 8, "Name": "KTone"}}]}
// [TPI 0x17e2] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 2, "Options": ["None"], "FieldList": 6113, "Name": "GrayOnlyParams", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 16}}
// [TPI 0x17e3] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "PreserveKPlaneParams", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x17e4] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4171, "FieldOffset": 0, "Name": "cmyk2cmyk"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 8, "Name": "hProofOutput"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 16, "Name": "cmyk2Lab"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4540, "FieldOffset": 24, "Name": "KTone"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4171, "FieldOffset": 32, "Name": "LabK2cmyk"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 65, "FieldOffset": 40, "Name": "MaxError"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 48, "Name": "hRoundTrip"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 65, "FieldOffset": 56, "Name": "MaxTAC"}}]}
// [TPI 0x17e5] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 8, "Options": ["None"], "FieldList": 6116, "Name": "PreserveKPlaneParams", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 64}}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: cmscnvrt.c.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===
