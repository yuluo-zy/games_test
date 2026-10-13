// Generated from PDB metadata by skeleton/_tools/fill_type_structures.py.
// Version: B
// 类型元数据；原作者源码文本/定义文件见逐项映射；不生成依赖实现。

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: aes_nohw.c.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB forward_declaration_only] AES_NOHW_BATCH; TPI 0x22ac
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "AES_NOHW_BATCH", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [8882]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] AES_NOHW_BATCH; TPI 0x22b2
// 原始定义文件映射: PDB_UDT_source_line
// size = 64; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 1, "Options": ["None"], "FieldList": 8881, "Name": "AES_NOHW_BATCH", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 64}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8881, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 8880, "FieldOffset": 0, "Name": "w"}}, "type_references": [{"index": 8880, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 35, "IndexType": 35, "Size": 64, "Name": ""}}, "nested_types": {"ElementType": {"index": 35, "spelling": "UInt64Quad", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [PDB UDT 源码行] {"ipi_index": 129779, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ring-0.16.20/crypto/fipsmodule/aes/aes_nohw.c", "line": 211, "module": null, "source_string_id": 129778}
// ------------------------------------------------------------------
// [PDB forward_declaration_only] AES_NOHW_SCHEDULE; TPI 0x22c5
// 原始定义文件映射: forward_definition_name_candidate; original declaration file unknown
// size = unknown; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "AES_NOHW_SCHEDULE", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// 前向定义候选（仅名称关联）: [8904]
// ------------------------------------------------------------------
// [PDB PDB_definition_record] AES_NOHW_SCHEDULE; TPI 0x22c8
// 原始定义文件映射: PDB_UDT_source_line
// size = 960; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 1, "Options": ["None"], "FieldList": 8903, "Name": "AES_NOHW_SCHEDULE", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 960}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8903, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 8902, "FieldOffset": 0, "Name": "keys"}}, "type_references": [{"index": 8902, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 8876, "IndexType": 35, "Size": 960, "Name": ""}}, "nested_types": {"ElementType": {"index": 8876, "kind": "LF_STRUCTURE", "spelling": "AES_NOHW_BATCH", "status": "PDB_record_name"}}}]}
// [PDB UDT 源码行] {"ipi_index": 129801, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ring-0.16.20/crypto/fipsmodule/aes/aes_nohw.c", "line": 218, "module": null, "source_string_id": 129778}
// ------------------------------------------------------------------
// [PDB PDB_definition_record] GFp_aes_nohw_ctr32_encrypt_blocks::<unnamed-tag>; TPI 0x22d7
// 原始定义文件映射: PDB_UDT_source_line
// size = 64; alignment / packing / Rust repr = unknown
// 定义记录: {"Kind": "LF_UNION", "Union": {"MemberCount": 2, "Options": ["None", "Scoped", "Sealed"], "FieldList": 8918, "Name": "GFp_aes_nohw_ctr32_encrypt_blocks::<unnamed-tag>", "UniqueName": "", "Size": 64}}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8918, "order": 0, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5275, "FieldOffset": 0, "Name": "u32"}}, "type_references": [{"index": 5275, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 117, "IndexType": 35, "Size": 64, "Name": ""}}, "nested_types": {"ElementType": {"index": 117, "spelling": "UInt32", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [字段 / 枚举值 / 类型关系] {"field_list_index": 8918, "order": 1, "kind": "LF_MEMBER", "record": {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6799, "FieldOffset": 0, "Name": "u8"}}, "type_references": [{"index": 6799, "kind": "LF_ARRAY", "status": "PDB_record", "record": {"Kind": "LF_ARRAY", "Array": {"ElementType": 32, "IndexType": 35, "Size": 64, "Name": ""}}, "nested_types": {"ElementType": {"index": 32, "spelling": "UnsignedCharacter", "status": "CodeView_simple_type; not Rust source spelling"}}}]}
// [PDB UDT 源码行] {"ipi_index": 129807, "original_path": "/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ring-0.16.20/crypto/fipsmodule/aes/aes_nohw.c", "line": 927, "module": null, "source_string_id": 129778}
// === 完整 TPI 结构关系引用记录；原文不转换成猜测的 Rust ABI ===
// [TPI 0x149b] {"Kind": "LF_ARRAY", "Array": {"ElementType": 117, "IndexType": 35, "Size": 64, "Name": ""}}
// [TPI 0x1a8f] {"Kind": "LF_ARRAY", "Array": {"ElementType": 32, "IndexType": 35, "Size": 64, "Name": ""}}
// [TPI 0x22ac] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "AES_NOHW_BATCH", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x22b0] {"Kind": "LF_ARRAY", "Array": {"ElementType": 35, "IndexType": 35, "Size": 64, "Name": ""}}
// [TPI 0x22b1] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 8880, "FieldOffset": 0, "Name": "w"}}]}
// [TPI 0x22b2] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 1, "Options": ["None"], "FieldList": 8881, "Name": "AES_NOHW_BATCH", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 64}}
// [TPI 0x22c5] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 0, "Options": ["None", "ForwardReference"], "FieldList": 0, "Name": "AES_NOHW_SCHEDULE", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 0}}
// [TPI 0x22c6] {"Kind": "LF_ARRAY", "Array": {"ElementType": 8876, "IndexType": 35, "Size": 960, "Name": ""}}
// [TPI 0x22c7] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 8902, "FieldOffset": 0, "Name": "keys"}}]}
// [TPI 0x22c8] {"Kind": "LF_STRUCTURE", "Class": {"MemberCount": 1, "Options": ["None"], "FieldList": 8903, "Name": "AES_NOHW_SCHEDULE", "UniqueName": "", "DerivationList": 0, "VTableShape": 0, "Size": 960}}
// [TPI 0x22d6] {"Kind": "LF_FIELDLIST", "FieldList": [{"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 5275, "FieldOffset": 0, "Name": "u32"}}, {"Kind": "LF_MEMBER", "DataMember": {"Attrs": 3, "Type": 6799, "FieldOffset": 0, "Name": "u8"}}]}
// [TPI 0x22d7] {"Kind": "LF_UNION", "Union": {"MemberCount": 2, "Options": ["None", "Scoped", "Sealed"], "FieldList": 8918, "Name": "GFp_aes_nohw_ctr32_encrypt_blocks::<unnamed-tag>", "UniqueName": "", "Size": 64}}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: aes_nohw.c.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===
