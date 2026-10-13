// Generated from PDB metadata by skeleton/_tools/fill_type_structures.py.
// Version: A
// 类型元数据；原作者源码文本/定义文件见逐项映射；不生成依赖实现。

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: bitstream.h.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB forward_declaration_only] BIT_DStream_t; TPI 0x1ffd
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "BIT_DStream_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [8194, 8693]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] <unnamed-tag>; TPI 0x200d
// 原始定义文件映射: PDB_UDT_source_line
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 4, "Options": ["None"], "FieldList": 8204, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8204, "order": 0, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 0, "Name": "BIT_DStream_unfinished"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8204, "order": 1, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1, "Name": "BIT_DStream_endOfBuffer"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8204, "order": 2, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 2, "Name": "BIT_DStream_completed"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8204, "order": 3, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 3, "Name": "BIT_DStream_overflow"}}, "type_references": []}
// [PDB UDT 源码行] {"ipi_index": 115077, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/legacy/zstd_v04.c", "line": 599, "module": null, "source_string_id": 115035}
// [PDB UDT 源码行] {"ipi_index": 115536, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/decompress/../common/bitstream.h", "line": 100, "module": null, "source_string_id": 115530}
// [PDB UDT 源码行] {"ipi_index": 115583, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/legacy/zstd_v02.c", "line": 327, "module": null, "source_string_id": 115572}
// [PDB UDT 源码行] {"ipi_index": 115672, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/legacy/zstd_v03.c", "line": 329, "module": null, "source_string_id": 115666}
// [PDB UDT 源码行] {"ipi_index": 121533, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/common/bitstream.h", "line": 100, "module": null, "source_string_id": 121529}
// ------------------------------------------------------------------
// [PDB PDB_definition_record] BIT_DStream_t; TPI 0x21f5
// 原始定义文件映射: PDB_UDT_source_line
// size = 40; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 5, "Options": ["None"], "FieldList": 8692, "Name": "BIT_DStream_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 40}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8692, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 0, "Name": "bitContainer"}}, "type_references": [{"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8692, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 8, "Name": "bitsConsumed"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8692, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4117, "FieldOffset": 16, "Name": "ptr"}}, "type_references": [{"index": 4117, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4106, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4106, "kind": "LF_MODIFIER", "status": "PDB_record", "record": {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 112, "Modifiers": ["None", "Const"]}}, "nested_types": {"ModifiedType": {"index": 112, "spelling": "NarrowCharacter", "status": "CodeView_simple_type; not Rust source spelling"}}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8692, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4117, "FieldOffset": 24, "Name": "start"}}, "type_references": [{"index": 4117, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4106, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4106, "kind": "LF_MODIFIER", "status": "PDB_record", "record": {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 112, "Modifiers": ["None", "Const"]}}, "nested_types": {"ModifiedType": {"index": 112, "spelling": "NarrowCharacter", "status": "CodeView_simple_type; not Rust source spelling"}}}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8692, "order": 4, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4117, "FieldOffset": 32, "Name": "limitPtr"}}, "type_references": [{"index": 4117, "kind": "LF_POINTER", "status": "PDB_record", "record": {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4106, "Attrs": 65548}}, "nested_types": {"ReferentType": {"index": 4106, "kind": "LF_MODIFIER", "status": "PDB_record", "record": {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 112, "Modifiers": ["None", "Const"]}}, "nested_types": {"ModifiedType": {"index": 112, "spelling": "NarrowCharacter", "status": "CodeView_simple_type; not Rust source spelling"}}}}}]}
// [PDB UDT 源码行] {"ipi_index": 115531, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/decompress/../common/bitstream.h", "line": 92, "module": null, "source_string_id": 115530}
// [PDB UDT 源码行] {"ipi_index": 121530, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/common/bitstream.h", "line": 92, "module": null, "source_string_id": 121529}
// === 完整 TPI 结构关系引用记录；原文不转换成猜测的 Rust ABI ===
// [TPI 0x100a] {"Kind": "LF_MODIFIER", "Modifier": {"ModifiedType": 112, "Modifiers": ["None", "Const"]}}
// [TPI 0x1015] {"Kind": "LF_POINTER", "Pointer": {"ReferentType": 4106, "Attrs": 65548}}
// [TPI 0x1ffd] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "BIT_DStream_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x200c] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 0, "Name": "BIT_DStream_unfinished"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1, "Name": "BIT_DStream_endOfBuffer"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 2, "Name": "BIT_DStream_completed"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 3, "Name": "BIT_DStream_overflow"}}]}
// [TPI 0x200d] {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 4, "Options": ["None"], "FieldList": 8204, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// [TPI 0x21f4] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 0, "Name": "bitContainer"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 8, "Name": "bitsConsumed"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4117, "FieldOffset": 16, "Name": "ptr"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4117, "FieldOffset": 24, "Name": "start"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4117, "FieldOffset": 32, "Name": "limitPtr"}}]}
// [TPI 0x21f5] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 5, "Options": ["None"], "FieldList": 8692, "Name": "BIT_DStream_t", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 40}}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: bitstream.h.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===
