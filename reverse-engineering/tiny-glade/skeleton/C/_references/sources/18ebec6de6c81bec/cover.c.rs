// Generated from PDB metadata by skeleton/_tools/fill_type_structures.py.
// Version: C
// 类型元数据；原作者源码文本/定义文件见逐项映射；不生成依赖实现。

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: cover.c.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB forward_declaration_only] COVER_map_s; TPI 0x2602
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "COVER_map_s", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [9737]
// ------------------------------------------------------------------
// [PDB forward_declaration_only] COVER_map_pair_t_s; TPI 0x2606
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "COVER_map_pair_t_s", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [9739]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] COVER_map_s; TPI 0x2609
// 原始定义文件映射: PDB_UDT_source_line
// size = 24; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 4, "Options": ["None"], "FieldList": 9736, "Name": "COVER_map_s", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 24}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 9736, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 9735, "FieldOffset": 0, "Name": "data"}}, "type_references": [{"index": 9735, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 9734, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 9734, "kind": "LF_STRUCTURE", "spelling": "COVER_map_pair_t_s", "status": "PDB_record_name"}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 9736, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 8, "Name": "sizeLog"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 9736, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 12, "Name": "size"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 9736, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 16, "Name": "sizeMask"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [PDB UDT 源码行] {"ipi_index": 147786, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/dictBuilder/cover.c", "line": 103, "module": null, "source_string_id": 147785}
// ------------------------------------------------------------------
// [PDB PDB_definition_record] COVER_map_pair_t_s; TPI 0x260b
// 原始定义文件映射: PDB_UDT_source_line
// size = 8; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 2, "Options": ["None"], "FieldList": 9738, "Name": "COVER_map_pair_t_s", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 8}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 9738, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 0, "Name": "key"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 9738, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 4, "Name": "value"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [PDB UDT 源码行] {"ipi_index": 147787, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/dictBuilder/cover.c", "line": 98, "module": null, "source_string_id": 147785}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] COVER_ctx_t; TPI 0x260e
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "COVER_ctx_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [9747]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] COVER_ctx_t; TPI 0x2613
// 原始定义文件映射: PDB_UDT_source_line
// size = 88; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 11, "Options": ["None"], "FieldList": 9746, "Name": "COVER_ctx_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 88}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 9746, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4781, "FieldOffset": 0, "Name": "samples"}}, "type_references": [{"index": 4781, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4780, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4780, "kind": "LF_MODIFIER", "status": "PDB_record", "record": {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 32, "Modifiers": ["None", "Const"]}}, "nested_types": {"ModifiedType": {"index": 32, "spelling": "UnsignedCharacter", "status": "CodeView_simple_type; not Rust source spelling"}}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 9746, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1571, "FieldOffset": 8, "Name": "offsets"}}, "type_references": [{"index": 1571, "spelling": "UInt64Quad / CodeView pointer mode 6", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 9746, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5422, "FieldOffset": 16, "Name": "samplesSizes"}}, "type_references": [{"index": 5422, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4921, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4921, "kind": "LF_MODIFIER", "status": "PDB_record", "record": {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 35, "Modifiers": ["None", "Const"]}}, "nested_types": {"ModifiedType": {"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 9746, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 24, "Name": "nbSamples"}}, "type_references": [{"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 9746, "order": 4, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 32, "Name": "nbTrainSamples"}}, "type_references": [{"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 9746, "order": 5, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 40, "Name": "nbTestSamples"}}, "type_references": [{"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 9746, "order": 6, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1653, "FieldOffset": 48, "Name": "suffix"}}, "type_references": [{"index": 1653, "spelling": "UInt32 / CodeView pointer mode 6", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 9746, "order": 7, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 56, "Name": "suffixSize"}}, "type_references": [{"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 9746, "order": 8, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1653, "FieldOffset": 64, "Name": "freqs"}}, "type_references": [{"index": 1653, "spelling": "UInt32 / CodeView pointer mode 6", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 9746, "order": 9, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1653, "FieldOffset": 72, "Name": "dmerAt"}}, "type_references": [{"index": 1653, "spelling": "UInt32 / CodeView pointer mode 6", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 9746, "order": 10, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 80, "Name": "d"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [PDB UDT 源码行] {"ipi_index": 147790, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/dictBuilder/cover.c", "line": 220, "module": null, "source_string_id": 147785}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] COVER_tryParameters_data_s; TPI 0x263a
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "COVER_tryParameters_data_s", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [9789]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] COVER_tryParameters_data_s; TPI 0x263d
// 原始定义文件映射: PDB_UDT_source_line
// size = 72; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 4, "Options": ["None"], "FieldList": 9788, "Name": "COVER_tryParameters_data_s", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 72}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 9788, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 9759, "FieldOffset": 0, "Name": "ctx"}}, "type_references": [{"index": 9759, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 9758, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 9758, "kind": "LF_MODIFIER", "status": "PDB_record", "record": {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 9742, "Modifiers": ["None", "Const"]}}, "nested_types": {"ModifiedType": {"index": 9742, "kind": "LF_STRUCTURE", "spelling": "COVER_ctx_t", "status": "PDB_record_name"}}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 9788, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 8847, "FieldOffset": 8, "Name": "best"}}, "type_references": [{"index": 8847, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 8842, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 8842, "kind": "LF_STRUCTURE", "spelling": "COVER_best_s", "status": "PDB_record_name"}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 9788, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 16, "Name": "dictBufferCapacity"}}, "type_references": [{"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 9788, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 8799, "FieldOffset": 24, "Name": "parameters"}}, "type_references": [{"index": 8799, "kind": "LF_STRUCTURE", "spelling": "ZDICT_cover_params_t", "status": "PDB_record_name"}]}
// [PDB UDT 源码行] {"ipi_index": 147819, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/dictBuilder/cover.c", "line": 1061, "module": null, "source_string_id": 147785}
// === 完整 TPI 结构关系引用记录；原文不转换成猜测的 Rust ABI ===
// [TPI 0x12ac] {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 32, "Modifiers": ["None", "Const"]}}
// [TPI 0x12ad] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4780, "Attrs": 65548}}
// [TPI 0x1339] {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 35, "Modifiers": ["None", "Const"]}}
// [TPI 0x152e] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4921, "Attrs": 65548}}
// [TPI 0x225f] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "ZDICT_cover_params_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x228a] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "COVER_best_s", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x228f] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 8842, "Attrs": 65548}}
// [TPI 0x2602] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "COVER_map_s", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x2606] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "COVER_map_pair_t_s", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x2607] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 9734, "Attrs": 65548}}
// [TPI 0x2608] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 9735, "FieldOffset": 0, "Name": "data"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 8, "Name": "sizeLog"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 12, "Name": "size"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 16, "Name": "sizeMask"}}]}
// [TPI 0x2609] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 4, "Options": ["None"], "FieldList": 9736, "Name": "COVER_map_s", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 24}}
// [TPI 0x260a] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 0, "Name": "key"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 4, "Name": "value"}}]}
// [TPI 0x260b] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 2, "Options": ["None"], "FieldList": 9738, "Name": "COVER_map_pair_t_s", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 8}}
// [TPI 0x260e] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "COVER_ctx_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x2612] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4781, "FieldOffset": 0, "Name": "samples"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1571, "FieldOffset": 8, "Name": "offsets"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5422, "FieldOffset": 16, "Name": "samplesSizes"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 24, "Name": "nbSamples"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 32, "Name": "nbTrainSamples"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 40, "Name": "nbTestSamples"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1653, "FieldOffset": 48, "Name": "suffix"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 56, "Name": "suffixSize"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1653, "FieldOffset": 64, "Name": "freqs"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1653, "FieldOffset": 72, "Name": "dmerAt"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 80, "Name": "d"}}]}
// [TPI 0x2613] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 11, "Options": ["None"], "FieldList": 9746, "Name": "COVER_ctx_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 88}}
// [TPI 0x261e] {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 9742, "Modifiers": ["None", "Const"]}}
// [TPI 0x261f] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 9758, "Attrs": 65548}}
// [TPI 0x263a] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "COVER_tryParameters_data_s", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x263c] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 9759, "FieldOffset": 0, "Name": "ctx"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 8847, "FieldOffset": 8, "Name": "best"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 16, "Name": "dictBufferCapacity"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 8799, "FieldOffset": 24, "Name": "parameters"}}]}
// [TPI 0x263d] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 4, "Options": ["None"], "FieldList": 9788, "Name": "COVER_tryParameters_data_s", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 72}}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: cover.c.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===
