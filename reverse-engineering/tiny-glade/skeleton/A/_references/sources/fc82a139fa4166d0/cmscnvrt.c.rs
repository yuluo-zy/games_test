// Generated from PDB metadata by skeleton/_tools/fill_type_structures.py.
// Version: A
// 类型元数据；原作者源码文本/定义文件见逐项映射；不生成依赖实现。

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: cmscnvrt.c.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB forward_declaration_only] _cms_intents_list; TPI 0x17e1
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "_cms_intents_list", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [6120]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] _cms_intents_list; TPI 0x17e8
// 原始定义文件映射: PDB_UDT_source_line
// size = 280; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 4, "Options": ["None"], "FieldList": 6119, "Name": "_cms_intents_list", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 280}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6119, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 0, "Name": "Intent"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6119, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4741, "FieldOffset": 4, "Name": "Description"}}, "type_references": [{"index": 4741, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 112, "IndexType": 35, "Size": 256, "Name": ""}}, "nested_types": {"ElementType": {"index": 112, "spelling": "NarrowCharacter", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6119, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6118, "FieldOffset": 264, "Name": "Link"}}, "type_references": [{"index": 6118, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 6117, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 6117, "kind": "LF_PROCEDURE", "status": "PDB_record", "record": {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 5067, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 7, "ArgumentList": 6116}}, "nested_types": {}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6119, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6114, "FieldOffset": 272, "Name": "Next"}}, "type_references": [{"index": 6114, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 6113, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 6113, "kind": "LF_STRUCTURE", "spelling": "_cms_intents_list", "status": "PDB_record_name"}}}]}
// [PDB UDT 源码行] {"ipi_index": 94783, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lcms2-sys-4.0.1/vendor/src/cmscnvrt.c", "line": 71, "module": null, "source_string_id": 94782}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] GrayOnlyParams; TPI 0x1803
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "GrayOnlyParams", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [6149]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] GrayOnlyParams; TPI 0x1805
// 原始定义文件映射: PDB_UDT_source_line
// size = 16; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 2, "Options": ["None"], "FieldList": 6148, "Name": "GrayOnlyParams", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 16}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6148, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5067, "FieldOffset": 0, "Name": "cmyk2cmyk"}}, "type_references": [{"index": 5067, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4982, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4982, "kind": "LF_STRUCTURE", "spelling": "_cmsPipeline_struct", "status": "PDB_record_name"}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6148, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4942, "FieldOffset": 8, "Name": "KTone"}}, "type_references": [{"index": 4942, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4941, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4941, "kind": "LF_STRUCTURE", "spelling": "_cms_curve_struct", "status": "PDB_record_name"}}}]}
// [PDB UDT 源码行] {"ipi_index": 94803, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lcms2-sys-4.0.1/vendor/src/cmscnvrt.c", "line": 691, "module": null, "source_string_id": 94782}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] PreserveKPlaneParams; TPI 0x1806
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "PreserveKPlaneParams", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [6152]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] PreserveKPlaneParams; TPI 0x1808
// 原始定义文件映射: PDB_UDT_source_line
// size = 64; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 8, "Options": ["None"], "FieldList": 6151, "Name": "PreserveKPlaneParams", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 64}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6151, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5067, "FieldOffset": 0, "Name": "cmyk2cmyk"}}, "type_references": [{"index": 5067, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4982, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4982, "kind": "LF_STRUCTURE", "spelling": "_cmsPipeline_struct", "status": "PDB_record_name"}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6151, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 8, "Name": "hProofOutput"}}, "type_references": [{"index": 1539, "spelling": "Void / CodeView pointer mode 6", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6151, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 16, "Name": "cmyk2Lab"}}, "type_references": [{"index": 1539, "spelling": "Void / CodeView pointer mode 6", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6151, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4942, "FieldOffset": 24, "Name": "KTone"}}, "type_references": [{"index": 4942, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4941, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4941, "kind": "LF_STRUCTURE", "spelling": "_cms_curve_struct", "status": "PDB_record_name"}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6151, "order": 4, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5067, "FieldOffset": 32, "Name": "LabK2cmyk"}}, "type_references": [{"index": 5067, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4982, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4982, "kind": "LF_STRUCTURE", "spelling": "_cmsPipeline_struct", "status": "PDB_record_name"}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6151, "order": 5, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 65, "FieldOffset": 40, "Name": "MaxError"}}, "type_references": [{"index": 65, "spelling": "Float64", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6151, "order": 6, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 48, "Name": "hRoundTrip"}}, "type_references": [{"index": 1539, "spelling": "Void / CodeView pointer mode 6", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 6151, "order": 7, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 65, "FieldOffset": 56, "Name": "MaxTAC"}}, "type_references": [{"index": 65, "spelling": "Float64", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [PDB UDT 源码行] {"ipi_index": 94805, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lcms2-sys-4.0.1/vendor/src/cmscnvrt.c", "line": 841, "module": null, "source_string_id": 94782}
// === 完整 TPI 结构关系引用记录；原文不转换成猜测的 Rust ABI ===
// [TPI 0x1209] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 1539, "Attrs": 65548}}
// [TPI 0x1274] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "_cmsContext_struct", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1275] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4724, "Attrs": 65548}}
// [TPI 0x1285] {"Kind": "LF_ARRAY", "Array": {"ElementType": 112, "IndexType": 35, "Size": 256, "Name": ""}}
// [TPI 0x134d] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "_cms_curve_struct", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x134e] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4941, "Attrs": 65548}}
// [TPI 0x1376] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "_cmsPipeline_struct", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x13cb] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4982, "Attrs": 65548}}
// [TPI 0x17e1] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "_cms_intents_list", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x17e2] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 6113, "Attrs": 65548}}
// [TPI 0x17e4] {"Kind": "LF_ARGLIST", "ArgList": {"ArgIndices": [4725, 117, 1653, 4617, 1652, 1601, 117]}}
// [TPI 0x17e5] {"Kind": "LF_PROCEDURE", "Procedure": {"ReturnType": 5067, "CallConv": "NearC", "Options": ["None"], "ParameterCount": 7, "ArgumentList": 6116}}
// [TPI 0x17e6] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 6117, "Attrs": 65548}}
// [TPI 0x17e7] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 0, "Name": "Intent"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4741, "FieldOffset": 4, "Name": "Description"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6118, "FieldOffset": 264, "Name": "Link"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6114, "FieldOffset": 272, "Name": "Next"}}]}
// [TPI 0x17e8] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 4, "Options": ["None"], "FieldList": 6119, "Name": "_cms_intents_list", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 280}}
// [TPI 0x1803] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "GrayOnlyParams", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1804] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5067, "FieldOffset": 0, "Name": "cmyk2cmyk"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4942, "FieldOffset": 8, "Name": "KTone"}}]}
// [TPI 0x1805] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 2, "Options": ["None"], "FieldList": 6148, "Name": "GrayOnlyParams", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 16}}
// [TPI 0x1806] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "PreserveKPlaneParams", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1807] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5067, "FieldOffset": 0, "Name": "cmyk2cmyk"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 8, "Name": "hProofOutput"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 16, "Name": "cmyk2Lab"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4942, "FieldOffset": 24, "Name": "KTone"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5067, "FieldOffset": 32, "Name": "LabK2cmyk"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 65, "FieldOffset": 40, "Name": "MaxError"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 48, "Name": "hRoundTrip"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 65, "FieldOffset": 56, "Name": "MaxTAC"}}]}
// [TPI 0x1808] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 8, "Options": ["None"], "FieldList": 6151, "Name": "PreserveKPlaneParams", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 64}}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: cmscnvrt.c.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===
