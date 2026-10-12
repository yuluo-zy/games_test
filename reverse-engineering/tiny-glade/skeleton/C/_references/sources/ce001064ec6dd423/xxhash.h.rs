// Generated from PDB metadata by skeleton/_tools/fill_type_structures.py.
// Version: C
// 类型元数据；原作者源码文本/定义文件见逐项映射；不生成依赖实现。

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: xxhash.h.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB forward_declaration_only] XXH_NAMESPACEXXH64_state_s; TPI 0x1240
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "XXH_NAMESPACEXXH64_state_s", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [4725]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] XXH_NAMESPACEXXH64_state_s; TPI 0x1275
// 原始定义文件映射: PDB_UDT_source_line
// size = 88; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 6, "Options": ["None"], "FieldList": 4724, "Name": "XXH_NAMESPACEXXH64_state_s", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 88}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4724, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 0, "Name": "total_len"}}, "type_references": [{"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4724, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4723, "FieldOffset": 8, "Name": "v"}}, "type_references": [{"index": 4723, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 35, "IndexType": 35, "Size": 32, "Name": ""}}, "nested_types": {"ElementType": {"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4724, "order": 2, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4723, "FieldOffset": 40, "Name": "mem64"}}, "type_references": [{"index": 4723, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 35, "IndexType": 35, "Size": 32, "Name": ""}}, "nested_types": {"ElementType": {"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4724, "order": 3, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 72, "Name": "memsize"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4724, "order": 4, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 76, "Name": "reserved32"}}, "type_references": [{"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 4724, "order": 5, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 80, "Name": "reserved64"}}, "type_references": [{"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}]}
// [PDB UDT 源码行] {"ipi_index": 101724, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/compress/../common/xxhash.h", "line": 990, "module": null, "source_string_id": 101723}
// [PDB UDT 源码行] {"ipi_index": 102222, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/decompress/../common/xxhash.h", "line": 990, "module": null, "source_string_id": 102221}
// [PDB UDT 源码行] {"ipi_index": 131723, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/dictBuilder/../common/xxhash.h", "line": 990, "module": null, "source_string_id": 131696}
// [PDB UDT 源码行] {"ipi_index": 132049, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/legacy/../common/xxhash.h", "line": 990, "module": null, "source_string_id": 132048}
// ------------------------------------------------------------------
// [PDB PDB_definition_record] <unnamed-tag>; TPI 0x1403
// 原始定义文件映射: PDB_UDT_source_line
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 2, "Options": ["None"], "FieldList": 5122, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5122, "order": 0, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 0, "Name": "XXH_aligned"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5122, "order": 1, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1, "Name": "XXH_unaligned"}}, "type_references": []}
// [PDB UDT 源码行] {"ipi_index": 101911, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/compress/../common/xxhash.h", "line": 1791, "module": null, "source_string_id": 101723}
// [PDB UDT 源码行] {"ipi_index": 102254, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/decompress/../common/xxhash.h", "line": 1791, "module": null, "source_string_id": 102221}
// [PDB UDT 源码行] {"ipi_index": 131697, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/dictBuilder/../common/xxhash.h", "line": 1791, "module": null, "source_string_id": 131696}
// [PDB UDT 源码行] {"ipi_index": 132086, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/legacy/../common/xxhash.h", "line": 1791, "module": null, "source_string_id": 132048}
// ------------------------------------------------------------------
// [PDB PDB_definition_record] <unnamed-tag>; TPI 0x140f
// 原始定义文件映射: PDB_UDT_source_line
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 2, "Options": ["None"], "FieldList": 5134, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5134, "order": 0, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 0, "Name": "XXH_NAMESPACEXXH_OK"}}, "type_references": []}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 5134, "order": 1, "kind": "LF_ENUMERATE", "record": {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1, "Name": "XXH_NAMESPACEXXH_ERROR"}}, "type_references": []}
// [PDB UDT 源码行] {"ipi_index": 101922, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/compress/../common/xxhash.h", "line": 332, "module": null, "source_string_id": 101723}
// [PDB UDT 源码行] {"ipi_index": 102247, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/decompress/../common/xxhash.h", "line": 332, "module": null, "source_string_id": 102221}
// [PDB UDT 源码行] {"ipi_index": 132076, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.1+zstd.1.5.2/zstd/lib/legacy/../common/xxhash.h", "line": 332, "module": null, "source_string_id": 132048}
// === 完整 TPI 结构关系引用记录；原文不转换成猜测的 Rust ABI ===
// [TPI 0x1240] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "XXH_NAMESPACEXXH64_state_s", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x1273] {"Kind": "LF_ARRAY", "Array": {"ElementType": 35, "IndexType": 35, "Size": 32, "Name": ""}}
// [TPI 0x1274] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 0, "Name": "total_len"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4723, "FieldOffset": 8, "Name": "v"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 4723, "FieldOffset": 40, "Name": "mem64"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 72, "Name": "memsize"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 117, "FieldOffset": 76, "Name": "reserved32"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 35, "FieldOffset": 80, "Name": "reserved64"}}]}
// [TPI 0x1275] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 6, "Options": ["None"], "FieldList": 4724, "Name": "XXH_NAMESPACEXXH64_state_s", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 88}}
// [TPI 0x1402] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 0, "Name": "XXH_aligned"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1, "Name": "XXH_unaligned"}}]}
// [TPI 0x1403] {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 2, "Options": ["None"], "FieldList": 5122, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// [TPI 0x140e] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 0, "Name": "XXH_NAMESPACEXXH_OK"}}, {"Kind": "LF_ENUMERATE", "Enumerator": {"Attrs": 3, "Value": 1, "Name": "XXH_NAMESPACEXXH_ERROR"}}]}
// [TPI 0x140f] {"Kind": "LF_ENUM", "Enum": {"NumEnumerators": 2, "Options": ["None"], "FieldList": 5134, "Name": "<unnamed-tag>", "UniqueName": "", "UnderlyingType": 116}}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: xxhash.h.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===
