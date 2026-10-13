// Generated from PDB metadata by skeleton/_tools/fill_type_structures.py.
// Version: C
// 类型元数据；原作者源码文本/定义文件见逐项映射；不生成依赖实现。

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: zstd_v01.c.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB forward_declaration_only] blockProperties_t; TPI 0x15cf
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "blockProperties_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [5585, 7040]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] <unnamed-tag>; TPI 0x1b7e
// 原始定义文件映射: PDB_UDT_source_line
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 4, "Options": ["None"], "FieldList": 7037, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7037, "order": 0, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 0, "Name": "bt_compressed"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7037, "order": 1, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1, "Name": "bt_raw"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7037, "order": 2, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 2, "Name": "bt_rle"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7037, "order": 3, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 3, "Name": "bt_end"}}, "type_references": []}
// [PDB UDT 源码行] {"ipi_index": 131796, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/legacy/zstd_v04.c", "line": 384, "module": null, "source_string_id": 131795}
// [PDB UDT 源码行] {"ipi_index": 131925, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/legacy/zstd_v06.c", "line": 477, "module": null, "source_string_id": 131899}
// [PDB UDT 源码行] {"ipi_index": 132043, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/legacy/zstd_v07.c", "line": 2715, "module": null, "source_string_id": 132015}
// [PDB UDT 源码行] {"ipi_index": 132190, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/legacy/zstd_v05.c", "line": 497, "module": null, "source_string_id": 132163}
// [PDB UDT 源码行] {"ipi_index": 132329, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/legacy/zstd_v02.c", "line": 2764, "module": null, "source_string_id": 132328}
// [PDB UDT 源码行] {"ipi_index": 132371, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/legacy/zstd_v01.c", "line": 1402, "module": null, "source_string_id": 132370}
// [PDB UDT 源码行] {"ipi_index": 132414, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/legacy/zstd_v03.c", "line": 2405, "module": null, "source_string_id": 132413}
// ------------------------------------------------------------------
// [PDB PDB_definition_record] blockProperties_t; TPI 0x1b80
// 原始定义文件映射: PDB_UDT_source_line
// size = 8; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 2, "Options": ["None"], "FieldList": 7039, "Name": "blockProperties_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 8}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7039, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7038, "FieldOffset": 0, "Name": "blockType"}}, "type_references": [{"index": 7038, "kind": "LF_ENUM", "spelling": "<unnamed-tag>", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7039, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 4, "Name": "origSize"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [PDB UDT 源码行] {"ipi_index": 131797, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/legacy/zstd_v04.c", "line": 2466, "module": null, "source_string_id": 131795}
// [PDB UDT 源码行] {"ipi_index": 131949, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/legacy/zstd_v06.c", "line": 3018, "module": null, "source_string_id": 131899}
// [PDB UDT 源码行] {"ipi_index": 132077, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/legacy/zstd_v07.c", "line": 3247, "module": null, "source_string_id": 132015}
// [PDB UDT 源码行] {"ipi_index": 132213, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/legacy/zstd_v05.c", "line": 2611, "module": null, "source_string_id": 132163}
// [PDB UDT 源码行] {"ipi_index": 132330, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/legacy/zstd_v02.c", "line": 2766, "module": null, "source_string_id": 132328}
// [PDB UDT 源码行] {"ipi_index": 132372, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/legacy/zstd_v01.c", "line": 1404, "module": null, "source_string_id": 132370}
// [PDB UDT 源码行] {"ipi_index": 132415, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/legacy/zstd_v03.c", "line": 2407, "module": null, "source_string_id": 132413}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] FSE_DState_t; TPI 0x1bb4
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "FSE_DState_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [7097]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] FSE_DState_t; TPI 0x1bb9
// 原始定义文件映射: PDB_UDT_source_line
// size = 16; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 2, "Options": ["None"], "FieldList": 7096, "Name": "FSE_DState_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 16}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7096, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 0, "Name": "state"}}, "type_references": [{"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7096, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4125, "FieldOffset": 8, "Name": "table"}}, "type_references": [{"index": 4125, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4124, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4124, "kind": "LF_MODIFIER", "status": "PDB_record", "record": {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 3, "Modifiers": ["None", "Const"]}}, "nested_types": {"ModifiedType": {"index": 3, "spelling": "Void", "status": "CodeView_simple_type; not Rust source spelling"}}}}}]}
// [PDB UDT 源码行] {"ipi_index": 131831, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/legacy/zstd_v04.c", "line": 848, "module": null, "source_string_id": 131795}
// [PDB UDT 源码行] {"ipi_index": 132338, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/legacy/zstd_v02.c", "line": 672, "module": null, "source_string_id": 132328}
// [PDB UDT 源码行] {"ipi_index": 132384, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/legacy/zstd_v01.c", "line": 85, "module": null, "source_string_id": 132370}
// [PDB UDT 源码行] {"ipi_index": 132418, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/legacy/zstd_v03.c", "line": 674, "module": null, "source_string_id": 132413}
// [PDB UDT 源码行] {"ipi_index": 144546, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/common/fse.h", "line": 437, "module": null, "source_string_id": 144545}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] seq_t; TPI 0x1bc1
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "seq_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [7109, 7260]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] seq_t; TPI 0x1bc5
// 原始定义文件映射: PDB_UDT_source_line
// size = 24; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None"], "FieldList": 7108, "Name": "seq_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 24}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7108, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 0, "Name": "litLength"}}, "type_references": [{"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7108, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 8, "Name": "offset"}}, "type_references": [{"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7108, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 16, "Name": "matchLength"}}, "type_references": [{"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [PDB UDT 源码行] {"ipi_index": 131838, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/legacy/zstd_v04.c", "line": 2789, "module": null, "source_string_id": 131795}
// [PDB UDT 源码行] {"ipi_index": 132340, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/legacy/zstd_v02.c", "line": 3022, "module": null, "source_string_id": 132328}
// [PDB UDT 源码行] {"ipi_index": 132394, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/legacy/zstd_v01.c", "line": 1673, "module": null, "source_string_id": 132370}
// [PDB UDT 源码行] {"ipi_index": 132420, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/legacy/zstd_v03.c", "line": 2663, "module": null, "source_string_id": 132413}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] seqState_t; TPI 0x1bc9
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "seqState_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [7118, 7263, 7384, 7521, 7602, 7707]
// ------------------------------------------------------------------
// [PDB forward_declaration_only] FSE_DTableHeader; TPI 0x1bfe
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "FSE_DTableHeader", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [7169]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] FSE_DTableHeader; TPI 0x1c01
// 原始定义文件映射: PDB_UDT_source_line
// size = 4; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 2, "Options": ["None"], "FieldList": 7168, "Name": "FSE_DTableHeader", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 4}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7168, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 33, "FieldOffset": 0, "Name": "tableLog"}}, "type_references": [{"index": 33, "spelling": "UInt16Short", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7168, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 33, "FieldOffset": 2, "Name": "fastMode"}}, "type_references": [{"index": 33, "spelling": "UInt16Short", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [PDB UDT 源码行] {"ipi_index": 131886, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/legacy/zstd_v04.c", "line": 874, "module": null, "source_string_id": 131795}
// [PDB UDT 源码行] {"ipi_index": 132361, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/legacy/zstd_v02.c", "line": 699, "module": null, "source_string_id": 132328}
// [PDB UDT 源码行] {"ipi_index": 132403, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/legacy/zstd_v01.c", "line": 390, "module": null, "source_string_id": 132370}
// [PDB UDT 源码行] {"ipi_index": 132433, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/legacy/zstd_v03.c", "line": 701, "module": null, "source_string_id": 132413}
// [PDB UDT 源码行] {"ipi_index": 144557, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/common/fse.h", "line": 591, "module": null, "source_string_id": 144545}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] FSE_decode_t; TPI 0x1c02
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "FSE_decode_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [7173]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] FSE_decode_t; TPI 0x1c05
// 原始定义文件映射: PDB_UDT_source_line
// size = 4; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None"], "FieldList": 7172, "Name": "FSE_decode_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 4}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7172, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 33, "FieldOffset": 0, "Name": "newState"}}, "type_references": [{"index": 33, "spelling": "UInt16Short", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7172, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 32, "FieldOffset": 2, "Name": "symbol"}}, "type_references": [{"index": 32, "spelling": "UnsignedCharacter", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7172, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 32, "FieldOffset": 3, "Name": "nbBits"}}, "type_references": [{"index": 32, "spelling": "UnsignedCharacter", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [PDB UDT 源码行] {"ipi_index": 131887, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/legacy/zstd_v04.c", "line": 879, "module": null, "source_string_id": 131795}
// [PDB UDT 源码行] {"ipi_index": 132362, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/legacy/zstd_v02.c", "line": 704, "module": null, "source_string_id": 132328}
// [PDB UDT 源码行] {"ipi_index": 132404, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/legacy/zstd_v01.c", "line": 125, "module": null, "source_string_id": 132370}
// [PDB UDT 源码行] {"ipi_index": 132434, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/legacy/zstd_v03.c", "line": 706, "module": null, "source_string_id": 132413}
// [PDB UDT 源码行] {"ipi_index": 144556, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/common/fse.h", "line": 596, "module": null, "source_string_id": 144545}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] FSE_DStream_t; TPI 0x1e06
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "FSE_DStream_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [7690]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] FSE_DStream_t; TPI 0x1e0a
// 原始定义文件映射: PDB_UDT_source_line
// size = 32; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 4, "Options": ["None"], "FieldList": 7090, "Name": "FSE_DStream_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 32}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7090, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 0, "Name": "bitContainer"}}, "type_references": [{"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7090, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 8, "Name": "bitsConsumed"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7090, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4428, "FieldOffset": 16, "Name": "ptr"}}, "type_references": [{"index": 4428, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4427, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4427, "kind": "LF_MODIFIER", "status": "PDB_record", "record": {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 112, "Modifiers": ["None", "Const"]}}, "nested_types": {"ModifiedType": {"index": 112, "spelling": "NarrowCharacter", "status": "CodeView_simple_type; not Rust source spelling"}}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7090, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4428, "FieldOffset": 24, "Name": "start"}}, "type_references": [{"index": 4428, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4427, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4427, "kind": "LF_MODIFIER", "status": "PDB_record", "record": {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 112, "Modifiers": ["None", "Const"]}}, "nested_types": {"ModifiedType": {"index": 112, "spelling": "NarrowCharacter", "status": "CodeView_simple_type; not Rust source spelling"}}}}}]}
// [PDB UDT 源码行] {"ipi_index": 132378, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/legacy/zstd_v01.c", "line": 77, "module": null, "source_string_id": 132370}
// ------------------------------------------------------------------
// [PDB PDB_definition_record] seqState_t; TPI 0x1e1b
// 原始定义文件映射: PDB_UDT_source_line
// size = 104; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 7, "Options": ["None"], "FieldList": 7706, "Name": "seqState_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 104}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7706, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7686, "FieldOffset": 0, "Name": "DStream"}}, "type_references": [{"index": 7686, "kind": "LF_STRUCTURE", "spelling": "FSE_DStream_t", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7706, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7092, "FieldOffset": 32, "Name": "stateLL"}}, "type_references": [{"index": 7092, "kind": "LF_STRUCTURE", "spelling": "FSE_DState_t", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7706, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7092, "FieldOffset": 48, "Name": "stateOffb"}}, "type_references": [{"index": 7092, "kind": "LF_STRUCTURE", "spelling": "FSE_DState_t", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7706, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7092, "FieldOffset": 64, "Name": "stateML"}}, "type_references": [{"index": 7092, "kind": "LF_STRUCTURE", "spelling": "FSE_DState_t", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7706, "order": 4, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 80, "Name": "prevOffset"}}, "type_references": [{"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7706, "order": 5, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4781, "FieldOffset": 88, "Name": "dumps"}}, "type_references": [{"index": 4781, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4780, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4780, "kind": "LF_MODIFIER", "status": "PDB_record", "record": {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 32, "Modifiers": ["None", "Const"]}}, "nested_types": {"ModifiedType": {"index": 32, "spelling": "UnsignedCharacter", "status": "CodeView_simple_type; not Rust source spelling"}}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7706, "order": 6, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4781, "FieldOffset": 96, "Name": "dumpsEnd"}}, "type_references": [{"index": 4781, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4780, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4780, "kind": "LF_MODIFIER", "status": "PDB_record", "record": {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 32, "Modifiers": ["None", "Const"]}}, "nested_types": {"ModifiedType": {"index": 32, "spelling": "UnsignedCharacter", "status": "CodeView_simple_type; not Rust source spelling"}}}}}]}
// [PDB UDT 源码行] {"ipi_index": 132395, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/legacy/zstd_v01.c", "line": 1679, "module": null, "source_string_id": 132370}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] HUF_DElt; TPI 0x1e1c
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "HUF_DElt", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [7713]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] HUF_DElt; TPI 0x1e21
// 原始定义文件映射: PDB_UDT_source_line
// size = 2; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 2, "Options": ["None"], "FieldList": 7134, "Name": "HUF_DElt", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 2}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7134, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 32, "FieldOffset": 0, "Name": "byte"}}, "type_references": [{"index": 32, "spelling": "UnsignedCharacter", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7134, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 32, "FieldOffset": 1, "Name": "nbBits"}}, "type_references": [{"index": 32, "spelling": "UnsignedCharacter", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [PDB UDT 源码行] {"ipi_index": 132398, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/legacy/zstd_v01.c", "line": 952, "module": null, "source_string_id": 132370}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] ZSTDv01_Dctx_s; TPI 0x1e22
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "ZSTDv01_Dctx_s", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [7719]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] ZSTDv01_Dctx_s; TPI 0x1e27
// 原始定义文件映射: PDB_UDT_source_line
// size = 10288; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 8, "Options": ["None"], "FieldList": 7718, "Name": "ZSTDv01_Dctx_s", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 10288}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7718, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7045, "FieldOffset": 0, "Name": "LLTable"}}, "type_references": [{"index": 7045, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 117, "IndexType": 35, "Size": 4100, "Name": ""}}, "nested_types": {"ElementType": {"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7718, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7046, "FieldOffset": 4100, "Name": "OffTable"}}, "type_references": [{"index": 7046, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 117, "IndexType": 35, "Size": 2052, "Name": ""}}, "nested_types": {"ElementType": {"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7718, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7045, "FieldOffset": 6152, "Name": "MLTable"}}, "type_references": [{"index": 7045, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 117, "IndexType": 35, "Size": 4100, "Name": ""}}, "nested_types": {"ElementType": {"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7718, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 10256, "Name": "previousDstEnd"}}, "type_references": [{"index": 1539, "spelling": "Void / CodeView pointer mode 6", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7718, "order": 4, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 10264, "Name": "base"}}, "type_references": [{"index": 1539, "spelling": "Void / CodeView pointer mode 6", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7718, "order": 5, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 10272, "Name": "expected"}}, "type_references": [{"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7718, "order": 6, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7038, "FieldOffset": 10280, "Name": "bType"}}, "type_references": [{"index": 7038, "kind": "LF_ENUM", "spelling": "<unnamed-tag>", "status": "PDB_record_name"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 7718, "order": 7, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 10284, "Name": "phase"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [PDB UDT 源码行] {"ipi_index": 132400, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/legacy/zstd_v01.c", "line": 1833, "module": null, "source_string_id": 132370}
// === 完整 TPI 结构关系引用记录；原文不转换成猜测的 Rust ABI ===
// [TPI 0x101c] {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 3, "Modifiers": ["None", "Const"]}}
// [TPI 0x101d] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4124, "Attrs": 65548}}
// [TPI 0x114b] {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 112, "Modifiers": ["None", "Const"]}}
// [TPI 0x114c] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4427, "Attrs": 65548}}
// [TPI 0x12ac] {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 32, "Modifiers": ["None", "Const"]}}
// [TPI 0x12ad] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4780, "Attrs": 65548}}
// [TPI 0x15cf] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "blockProperties_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1b7d] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 0, "Name": "bt_compressed"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1, "Name": "bt_raw"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 2, "Name": "bt_rle"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 3, "Name": "bt_end"}}]}
// [TPI 0x1b7e] {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 4, "Options": ["None"], "FieldList": 7037, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// [TPI 0x1b7f] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7038, "FieldOffset": 0, "Name": "blockType"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 4, "Name": "origSize"}}]}
// [TPI 0x1b80] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 2, "Options": ["None"], "FieldList": 7039, "Name": "blockProperties_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 8}}
// [TPI 0x1b85] {"Kind": "LF_ARRAY", "Array": {"ElementType": 117, "IndexType": 35, "Size": 4100, "Name": ""}}
// [TPI 0x1b86] {"Kind": "LF_ARRAY", "Array": {"ElementType": 117, "IndexType": 35, "Size": 2052, "Name": ""}}
// [TPI 0x1bb2] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 0, "Name": "bitContainer"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 8, "Name": "bitsConsumed"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4428, "FieldOffset": 16, "Name": "ptr"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4428, "FieldOffset": 24, "Name": "start"}}]}
// [TPI 0x1bb4] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "FSE_DState_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1bb8] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 0, "Name": "state"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4125, "FieldOffset": 8, "Name": "table"}}]}
// [TPI 0x1bb9] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 2, "Options": ["None"], "FieldList": 7096, "Name": "FSE_DState_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 16}}
// [TPI 0x1bc1] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "seq_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1bc4] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 0, "Name": "litLength"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 8, "Name": "offset"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 16, "Name": "matchLength"}}]}
// [TPI 0x1bc5] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None"], "FieldList": 7108, "Name": "seq_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 24}}
// [TPI 0x1bc9] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "seqState_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1bde] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 32, "FieldOffset": 0, "Name": "byte"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 32, "FieldOffset": 1, "Name": "nbBits"}}]}
// [TPI 0x1bfe] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "FSE_DTableHeader", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1c00] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 33, "FieldOffset": 0, "Name": "tableLog"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 33, "FieldOffset": 2, "Name": "fastMode"}}]}
// [TPI 0x1c01] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 2, "Options": ["None"], "FieldList": 7168, "Name": "FSE_DTableHeader", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 4}}
// [TPI 0x1c02] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "FSE_decode_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1c04] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 33, "FieldOffset": 0, "Name": "newState"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 32, "FieldOffset": 2, "Name": "symbol"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 32, "FieldOffset": 3, "Name": "nbBits"}}]}
// [TPI 0x1c05] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 3, "Options": ["None"], "FieldList": 7172, "Name": "FSE_decode_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 4}}
// [TPI 0x1e06] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "FSE_DStream_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1e0a] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 4, "Options": ["None"], "FieldList": 7090, "Name": "FSE_DStream_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 32}}
// [TPI 0x1e1a] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7686, "FieldOffset": 0, "Name": "DStream"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7092, "FieldOffset": 32, "Name": "stateLL"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7092, "FieldOffset": 48, "Name": "stateOffb"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7092, "FieldOffset": 64, "Name": "stateML"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 80, "Name": "prevOffset"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4781, "FieldOffset": 88, "Name": "dumps"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4781, "FieldOffset": 96, "Name": "dumpsEnd"}}]}
// [TPI 0x1e1b] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 7, "Options": ["None"], "FieldList": 7706, "Name": "seqState_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 104}}
// [TPI 0x1e1c] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "HUF_DElt", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1e21] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 2, "Options": ["None"], "FieldList": 7134, "Name": "HUF_DElt", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 2}}
// [TPI 0x1e22] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "ZSTDv01_Dctx_s", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1e26] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7045, "FieldOffset": 0, "Name": "LLTable"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7046, "FieldOffset": 4100, "Name": "OffTable"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7045, "FieldOffset": 6152, "Name": "MLTable"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 10256, "Name": "previousDstEnd"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 1539, "FieldOffset": 10264, "Name": "base"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 10272, "Name": "expected"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 7038, "FieldOffset": 10280, "Name": "bType"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 10284, "Name": "phase"}}]}
// [TPI 0x1e27] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 8, "Options": ["None"], "FieldList": 7718, "Name": "ZSTDv01_Dctx_s", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 10288}}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: zstd_v01.c.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===
