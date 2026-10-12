// Generated from PDB metadata by skeleton/_tools/fill_type_structures.py.
// Version: C
// 类型元数据；原作者源码文本/定义文件见逐项映射；不生成依赖实现。

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: fastcover.c.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB forward_declaration_only] FASTCOVER_ctx_t; TPI 0x2267
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "FASTCOVER_ctx_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [8813]
// ------------------------------------------------------------------
// [PDB forward_declaration_only] FASTCOVER_accel_t; TPI 0x226b
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "FASTCOVER_accel_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [8815]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] FASTCOVER_ctx_t; TPI 0x226d
// 原始定义文件映射: PDB_UDT_source_line
// size = 80; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 11, "Options": ["None"], "FieldList": 8812, "Name": "FASTCOVER_ctx_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 80}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8812, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4781, "FieldOffset": 0, "Name": "samples"}}, "type_references": [{"index": 4781, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4780, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4780, "kind": "LF_MODIFIER", "status": "PDB_record", "record": {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 32, "Modifiers": ["None", "Const"]}}, "nested_types": {"ModifiedType": {"index": 32, "spelling": "UnsignedCharacter", "status": "CodeView_simple_type; not Rust source spelling"}}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8812, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1571, "FieldOffset": 8, "Name": "offsets"}}, "type_references": [{"index": 1571, "spelling": "UInt64Quad / CodeView pointer mode 6", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8812, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5422, "FieldOffset": 16, "Name": "samplesSizes"}}, "type_references": [{"index": 5422, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4921, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4921, "kind": "LF_MODIFIER", "status": "PDB_record", "record": {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 35, "Modifiers": ["None", "Const"]}}, "nested_types": {"ModifiedType": {"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8812, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 24, "Name": "nbSamples"}}, "type_references": [{"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8812, "order": 4, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 32, "Name": "nbTrainSamples"}}, "type_references": [{"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8812, "order": 5, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 40, "Name": "nbTestSamples"}}, "type_references": [{"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8812, "order": 6, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 48, "Name": "nbDmers"}}, "type_references": [{"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8812, "order": 7, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1653, "FieldOffset": 56, "Name": "freqs"}}, "type_references": [{"index": 1653, "spelling": "UInt32 / CodeView pointer mode 6", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8812, "order": 8, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 64, "Name": "d"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8812, "order": 9, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 68, "Name": "f"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8812, "order": 10, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 8811, "FieldOffset": 72, "Name": "accelParams"}}, "type_references": [{"index": 8811, "kind": "LF_STRUCTURE", "spelling": "FASTCOVER_accel_t", "status": "PDB_record_name"}]}
// [PDB UDT 源码行] {"ipi_index": 144611, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/dictBuilder/fastcover.c", "line": 127, "module": null, "source_string_id": 144610}
// ------------------------------------------------------------------
// [PDB PDB_definition_record] FASTCOVER_accel_t; TPI 0x226f
// 原始定义文件映射: PDB_UDT_source_line
// size = 8; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 2, "Options": ["None"], "FieldList": 8814, "Name": "FASTCOVER_accel_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 8}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8814, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 0, "Name": "finalize"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8814, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 4, "Name": "skip"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [PDB UDT 源码行] {"ipi_index": 144612, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/dictBuilder/fastcover.c", "line": 103, "module": null, "source_string_id": 144610}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] FASTCOVER_tryParameters_data_s; TPI 0x228d
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "FASTCOVER_tryParameters_data_s", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [8849]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] FASTCOVER_tryParameters_data_s; TPI 0x2291
// 原始定义文件映射: PDB_UDT_source_line
// size = 72; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 4, "Options": ["None"], "FieldList": 8848, "Name": "FASTCOVER_tryParameters_data_s", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 72}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8848, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 8817, "FieldOffset": 0, "Name": "ctx"}}, "type_references": [{"index": 8817, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 8816, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 8816, "kind": "LF_MODIFIER", "status": "PDB_record", "record": {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 8807, "Modifiers": ["None", "Const"]}}, "nested_types": {"ModifiedType": {"index": 8807, "kind": "LF_STRUCTURE", "spelling": "FASTCOVER_ctx_t", "status": "PDB_record_name"}}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8848, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 8847, "FieldOffset": 8, "Name": "best"}}, "type_references": [{"index": 8847, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 8842, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 8842, "kind": "LF_STRUCTURE", "spelling": "COVER_best_s", "status": "PDB_record_name"}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8848, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 16, "Name": "dictBufferCapacity"}}, "type_references": [{"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8848, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 8799, "FieldOffset": 24, "Name": "parameters"}}, "type_references": [{"index": 8799, "kind": "LF_STRUCTURE", "spelling": "ZDICT_cover_params_t", "status": "PDB_record_name"}]}
// [PDB UDT 源码行] {"ipi_index": 144626, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/dictBuilder/fastcover.c", "line": 460, "module": null, "source_string_id": 144610}
// === 完整 TPI 结构关系引用记录；原文不转换成猜测的 Rust ABI ===
// [TPI 0x12ac] {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 32, "Modifiers": ["None", "Const"]}}
// [TPI 0x12ad] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4780, "Attrs": 65548}}
// [TPI 0x1339] {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 35, "Modifiers": ["None", "Const"]}}
// [TPI 0x152e] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4921, "Attrs": 65548}}
// [TPI 0x225f] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "ZDICT_cover_params_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x2267] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "FASTCOVER_ctx_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x226b] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "FASTCOVER_accel_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x226c] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4781, "FieldOffset": 0, "Name": "samples"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1571, "FieldOffset": 8, "Name": "offsets"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5422, "FieldOffset": 16, "Name": "samplesSizes"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 24, "Name": "nbSamples"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 32, "Name": "nbTrainSamples"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 40, "Name": "nbTestSamples"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 48, "Name": "nbDmers"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1653, "FieldOffset": 56, "Name": "freqs"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 64, "Name": "d"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 68, "Name": "f"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 8811, "FieldOffset": 72, "Name": "accelParams"}}]}
// [TPI 0x226d] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 11, "Options": ["None"], "FieldList": 8812, "Name": "FASTCOVER_ctx_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 80}}
// [TPI 0x226e] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 0, "Name": "finalize"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 4, "Name": "skip"}}]}
// [TPI 0x226f] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 2, "Options": ["None"], "FieldList": 8814, "Name": "FASTCOVER_accel_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 8}}
// [TPI 0x2270] {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 8807, "Modifiers": ["None", "Const"]}}
// [TPI 0x2271] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 8816, "Attrs": 65548}}
// [TPI 0x228a] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "COVER_best_s", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x228d] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "FASTCOVER_tryParameters_data_s", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x228f] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 8842, "Attrs": 65548}}
// [TPI 0x2290] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 8817, "FieldOffset": 0, "Name": "ctx"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 8847, "FieldOffset": 8, "Name": "best"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 16, "Name": "dictBufferCapacity"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 8799, "FieldOffset": 24, "Name": "parameters"}}]}
// [TPI 0x2291] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 4, "Options": ["None"], "FieldList": 8848, "Name": "FASTCOVER_tryParameters_data_s", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 72}}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: fastcover.c.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===
