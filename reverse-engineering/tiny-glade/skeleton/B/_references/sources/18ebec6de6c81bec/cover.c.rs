// Generated from PDB metadata by skeleton/_tools/fill_type_structures.py.
// Version: B
// 类型元数据；原作者源码文本/定义文件见逐项映射；不生成依赖实现。

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: cover.c.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB forward_declaration_only] COVER_map_s; TPI 0x2606
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "COVER_map_s", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [9741]
// ------------------------------------------------------------------
// [PDB forward_declaration_only] COVER_map_pair_t_s; TPI 0x260a
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "COVER_map_pair_t_s", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [9743]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] COVER_map_s; TPI 0x260d
// 原始定义文件映射: PDB_UDT_source_line
// size = 24; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 4, "Options": ["None"], "FieldList": 9740, "Name": "COVER_map_s", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 24}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 9740, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 9739, "FieldOffset": 0, "Name": "data"}}, "type_references": [{"index": 9739, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 9738, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 9738, "kind": "LF_STRUCTURE", "spelling": "COVER_map_pair_t_s", "status": "PDB_record_name"}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 9740, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 8, "Name": "sizeLog"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 9740, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 12, "Name": "size"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 9740, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 16, "Name": "sizeMask"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [PDB UDT 源码行] {"ipi_index": 134935, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/dictBuilder/cover.c", "line": 103, "module": null, "source_string_id": 134934}
// ------------------------------------------------------------------
// [PDB PDB_definition_record] COVER_map_pair_t_s; TPI 0x260f
// 原始定义文件映射: PDB_UDT_source_line
// size = 8; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 2, "Options": ["None"], "FieldList": 9742, "Name": "COVER_map_pair_t_s", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 8}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 9742, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 0, "Name": "key"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 9742, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 4, "Name": "value"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [PDB UDT 源码行] {"ipi_index": 134936, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/dictBuilder/cover.c", "line": 98, "module": null, "source_string_id": 134934}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] COVER_ctx_t; TPI 0x2612
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "COVER_ctx_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [9751]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] COVER_ctx_t; TPI 0x2617
// 原始定义文件映射: PDB_UDT_source_line
// size = 88; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 11, "Options": ["None"], "FieldList": 9750, "Name": "COVER_ctx_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 88}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 9750, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4711, "FieldOffset": 0, "Name": "samples"}}, "type_references": [{"index": 4711, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4710, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4710, "kind": "LF_MODIFIER", "status": "PDB_record", "record": {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 32, "Modifiers": ["None", "Const"]}}, "nested_types": {"ModifiedType": {"index": 32, "spelling": "UnsignedCharacter", "status": "CodeView_simple_type; not Rust source spelling"}}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 9750, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1571, "FieldOffset": 8, "Name": "offsets"}}, "type_references": [{"index": 1571, "spelling": "UInt64Quad / CodeView pointer mode 6", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 9750, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6292, "FieldOffset": 16, "Name": "samplesSizes"}}, "type_references": [{"index": 6292, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4679, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4679, "kind": "LF_MODIFIER", "status": "PDB_record", "record": {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 35, "Modifiers": ["None", "Const"]}}, "nested_types": {"ModifiedType": {"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 9750, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 24, "Name": "nbSamples"}}, "type_references": [{"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 9750, "order": 4, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 32, "Name": "nbTrainSamples"}}, "type_references": [{"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 9750, "order": 5, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 40, "Name": "nbTestSamples"}}, "type_references": [{"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 9750, "order": 6, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1653, "FieldOffset": 48, "Name": "suffix"}}, "type_references": [{"index": 1653, "spelling": "UInt32 / CodeView pointer mode 6", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 9750, "order": 7, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 56, "Name": "suffixSize"}}, "type_references": [{"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 9750, "order": 8, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1653, "FieldOffset": 64, "Name": "freqs"}}, "type_references": [{"index": 1653, "spelling": "UInt32 / CodeView pointer mode 6", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 9750, "order": 9, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1653, "FieldOffset": 72, "Name": "dmerAt"}}, "type_references": [{"index": 1653, "spelling": "UInt32 / CodeView pointer mode 6", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 9750, "order": 10, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 80, "Name": "d"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [PDB UDT 源码行] {"ipi_index": 134939, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/dictBuilder/cover.c", "line": 220, "module": null, "source_string_id": 134934}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] COVER_tryParameters_data_s; TPI 0x263e
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "COVER_tryParameters_data_s", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [9793]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] COVER_tryParameters_data_s; TPI 0x2641
// 原始定义文件映射: PDB_UDT_source_line
// size = 72; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 4, "Options": ["None"], "FieldList": 9792, "Name": "COVER_tryParameters_data_s", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 72}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 9792, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 9763, "FieldOffset": 0, "Name": "ctx"}}, "type_references": [{"index": 9763, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 9762, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 9762, "kind": "LF_MODIFIER", "status": "PDB_record", "record": {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 9746, "Modifiers": ["None", "Const"]}}, "nested_types": {"ModifiedType": {"index": 9746, "kind": "LF_STRUCTURE", "spelling": "COVER_ctx_t", "status": "PDB_record_name"}}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 9792, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 8858, "FieldOffset": 8, "Name": "best"}}, "type_references": [{"index": 8858, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 8853, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 8853, "kind": "LF_STRUCTURE", "spelling": "COVER_best_s", "status": "PDB_record_name"}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 9792, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 16, "Name": "dictBufferCapacity"}}, "type_references": [{"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 9792, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 8810, "FieldOffset": 24, "Name": "parameters"}}, "type_references": [{"index": 8810, "kind": "LF_STRUCTURE", "spelling": "ZDICT_cover_params_t", "status": "PDB_record_name"}]}
// [PDB UDT 源码行] {"ipi_index": 134968, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/dictBuilder/cover.c", "line": 1061, "module": null, "source_string_id": 134934}
// === 完整 TPI 结构关系引用记录；原文不转换成猜测的 Rust ABI ===
// [TPI 0x1247] {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 35, "Modifiers": ["None", "Const"]}}
// [TPI 0x1266] {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 32, "Modifiers": ["None", "Const"]}}
// [TPI 0x1267] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4710, "Attrs": 65548}}
// [TPI 0x1894] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4679, "Attrs": 65548}}
// [TPI 0x226a] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "ZDICT_cover_params_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x2295] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "COVER_best_s", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x229a] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 8853, "Attrs": 65548}}
// [TPI 0x2606] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "COVER_map_s", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x260a] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "COVER_map_pair_t_s", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x260b] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 9738, "Attrs": 65548}}
// [TPI 0x260c] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 9739, "FieldOffset": 0, "Name": "data"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 8, "Name": "sizeLog"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 12, "Name": "size"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 16, "Name": "sizeMask"}}]}
// [TPI 0x260d] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 4, "Options": ["None"], "FieldList": 9740, "Name": "COVER_map_s", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 24}}
// [TPI 0x260e] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 0, "Name": "key"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 4, "Name": "value"}}]}
// [TPI 0x260f] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 2, "Options": ["None"], "FieldList": 9742, "Name": "COVER_map_pair_t_s", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 8}}
// [TPI 0x2612] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "COVER_ctx_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x2616] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4711, "FieldOffset": 0, "Name": "samples"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1571, "FieldOffset": 8, "Name": "offsets"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6292, "FieldOffset": 16, "Name": "samplesSizes"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 24, "Name": "nbSamples"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 32, "Name": "nbTrainSamples"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 40, "Name": "nbTestSamples"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1653, "FieldOffset": 48, "Name": "suffix"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 56, "Name": "suffixSize"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1653, "FieldOffset": 64, "Name": "freqs"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1653, "FieldOffset": 72, "Name": "dmerAt"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 80, "Name": "d"}}]}
// [TPI 0x2617] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 11, "Options": ["None"], "FieldList": 9750, "Name": "COVER_ctx_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 88}}
// [TPI 0x2622] {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 9746, "Modifiers": ["None", "Const"]}}
// [TPI 0x2623] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 9762, "Attrs": 65548}}
// [TPI 0x263e] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "COVER_tryParameters_data_s", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x2640] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 9763, "FieldOffset": 0, "Name": "ctx"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 8858, "FieldOffset": 8, "Name": "best"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 16, "Name": "dictBufferCapacity"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 8810, "FieldOffset": 24, "Name": "parameters"}}]}
// [TPI 0x2641] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 4, "Options": ["None"], "FieldList": 9792, "Name": "COVER_tryParameters_data_s", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 72}}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: cover.c.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===
