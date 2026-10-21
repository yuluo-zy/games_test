# 原历史v27 Terrain f32数字合同独立复核

## 结论与断言方向

本次不是已发现的生产坐标损坏。当前测试把原生f32字段在不同JSON表示路径上的f64 Number直接相等，当成原字段是否保留的判定，合同过强。失败日志 `/private/tmp/tg-original-histories-wire-v3-20261021.log` 与当前 `original_gameplay_chain.rs` a7ee443逐实参核：`assert_original_fields_retained(&original,&encoded,...)`，叶子assert_eq第一个实参是original，故left=原资产紧凑数字3.9437943，right=生产serializer写后再解析的3.9437942504882817；不能反写方向。

原资产Version27、full/03-halftimber-roof entry53是Terrain桶key0，完整原源文件SHA1ae8c329993ad59e7b3c2a69ac1d85f299c5c3cd0daf2c804beb9a68cba5e9a3。其after是字符串key对象，不是数组；路径中的0是装饰物ID。

## 精确原生类型与生产数据流

实际当前读取类型 `snapshot.rs::DecoratorTerrainDstJson` 的 `pos_xz:[f32;2]`、`yaw_radians:f32`，decorator_dst_fields将两坐标to_bits打包入原dst_wall两个u32，将yaw打包入dst_secondary低u32；业务 `TerrainDecoratorDst.position_xz` 也是glam::Vec2。

独立原B类型证据（两完整范围共1041B，见NATIVE_TYPE_BYTE_PROOF_V1.json）：原新变体Value serializer1415910d0/635B PDB1628:46832，14159116f直接调140a4b5a0/406B PDB546:78772。后者140a4b5c4用movss读取yaw+8，140a4b5ff调用原PDB1628:151748 `serialize_field<glam::f32::vec2::Vec2>`，140a4b698调用原PDB1628:136156 `serialize_field<f32>`。两字段名原PE字符串142acf509/6=`pos_xz`、142acf50f/11=`yaw_radians`。PDB没有字段定义时不凭结构名伪造布局；此处类型来自真实序列化专化。

当前正式写路是 SavedMultiEdit::serialize → DecoratorHistoryEdit::serialize → snapshot::decorator_history_state_json → decorator_info_json → json!([f(0),f(4)])。json!使用to_value，锁定serde_json1.0.85的value/ser.rs133明确 `serialize_f32 -> serialize_f64(value as f64)`。因此即使外层serde_json::to_vec(&entry)走CompactFormatter，嵌套Value内坐标也已经是f64；不是所有f32都会像直接compact-f32 writer那样输出原资产短文本。save_game_to_memory本身也按原证先将MultiEdit转Value再写Compact，不应为使测试文本相等改生产为另一协议。

## 位级数值合同

- left原3.9437943，按IEEE binary64阅读为400f8ce406a535af；原字段读回binary32为407c6720。
- 407c6720精确数值3.94379425048828125；无算术地扩binary64为400f8ce400000000。
- 当前right日志解析后3.9437942504882817，binary64为400f8ce400000001，恰比上一项高一个f64 ULP；收窄到源binary32仍407c6720。

serde_json1.0.85/de.rs561默认非float_roundtrip路径使用(significand as f64)/POW10。当前Cargo明确pin=1.0.85，所有现存目标serde_json fingerprint只启用default/std，未启float_roundtrip。解析例如39437942504882813e-16时，u64先转f64得到整数39437942504882816，再除1e16，确切产生400f8ce400000001，与日志一致。这是当前默认十进制解析合同；Python按相同IEEE运算得到一致位型仅作静态算式辅助，不冒作Rust实测。原实际日志已证明右值，但未从该失败过程中捕获临时wire字符串，所以不捏造它最后一位文本。

## 最窄候选与验证边界

仅修改集成测试比较器：在严格层级 `decorators/diffs/{row}/1/{before|after}/{u32-id}/dst/Terrain/` 内，`pos_xz/0`、`pos_xz/1`、`yaw_radians` 这三个有原B类型证据的叶子，将两侧Number按当前f32解码后逐to_bits相等，且两侧必须finite。所有字段存在性、变体、HashMap键、数组长度与顺序、其它数字和未知字段依然执行原assert。没有epsilon、没有丢字段、没有修改生产序列化器、没有改依赖feature。

同entry53的pos1与yaw使用同一原类型/写路，一并覆盖才能避免只特判报告pos0然后顺次再失败。其它Dst/标量可能有相同表示问题，但本最窄包不以名字猜宽到全部Number，也不在未提供相应原证时把其它域都归一为f32；若后续完整corpus遇到它们，需按其确切schema单独证实类型后扩展，不降低ID等整数精度。

三个新测试：①保留entry53原数字文本，经真实SavedMultiEdit读取/当前生产序列化，检查源407c6720、Value宽化000、Compact解析001及完整原字段检查和auxiliary重新加载；②相邻一个f32 bit与-0/+0变化必须失败；③未知字段/Roof域/越界数组位和大u32 ID差异仍失败，before/after三合法叶子才能按源f32比较。测试只是准备，实际Cargo/全33资产corpus仍由root执行；本报告不声明测试或全部游戏链通过。

源码候选保持旧GPU/业务集成测试与全部已有断言；唯一现有行为变更是上述数字比较叶。rustfmt stdin语法解析与隔离F0 patch/hash已完成，未修改生产。
