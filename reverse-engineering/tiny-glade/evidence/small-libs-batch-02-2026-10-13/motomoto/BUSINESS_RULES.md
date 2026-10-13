# motomoto：batch-02 反编译复核与消费者映射

状态：batch-01 已交付 Reader 173 原指令案例零差异与 Writer/Drop Cursor 候选路径。本轮（batch-02）用 Ghidra shared worker 对 B patched EXE 反编译 `add_chunk` 两单态、`ChunkWriter` Drop、`Reader::iter`、`impl$9::next` 及存档消费者入口，逐条核对候选实现；并补一条直接编码反编译规则的字节级测试。未修改原库 API。

## 本轮反编译核对结果（vs 候选实现）

| 原函数 | 地址 | 反编译事实 | 候选实现 | 结论 |
|---|---|---|---|---|
| `Writer::add_chunk<Cursor<&mut Vec>, &str>` | 0x141552690 | 先取并清空 `deferred_error`；非空则返回 Err；再 stream_position → 写 tag u64 LE → 写 0 长度 → 返回 ChunkWriter | `add_chunk`：`deferred_error.take()` 先行，`stream_position`，两次 `write_u64` | 一致 |
| `Writer::add_chunk<Cursor<&mut Vec>, u64>` | 0x141552820 | 同上（tag 已在寄存器，无字符串转换） | 同一 `add_chunk` 泛型 u64 单态 | 一致 |
| `drop_in_place<ChunkWriter<Cursor<&mut Vec>>>` | 0x14087abf0 | Drop 记录 end=当前位；写 `end-start-16` 到 data+8+start；恢复位=end；随后访问 Writer 错误槽：存在则 drop 原 Error 并清空（成功路径清空槽） | `seek_back_and_write_length`（end, seek start+8, 写 end-start-16, seek end）+ `Drop` 将槽替换为 `.err()` | 一致（Cursor 路径 seek 不会失败，等价于清空槽） |
| `Reader::iter` | 0x142080d50 | 复制 slice ptr/len，cursor position=0 | `Reader::iter` 新建 `Cursor::new(data)` | 一致 |
| `impl$9::next`（try_next 单态） | 0x142080d70 | position+16 >= len → 返回 None 不消费尾部；读 tag@pos、length@pos+8；position+=16；end=position+length；溢出→错误；end>len→ErrorKind=UnexpectedEof(0x2500000003)；end<start→slice_index_order_fail panic | `try_next`：`start=position.wrapping_add(16)` 越界→None；`read_u64`×2；`SeekFrom::Current(length as i64)`；`end>len`→UnexpectedEof；`&data[start..end]` panic | 一致 |

结论：候选实现与 B 反编译一致，无需改动库逻辑；本轮改动仅为测试新增（字节级 header 布局 + 尾部不足 16 字节忽略）。

## 存档消费者映射（反编译确认，不跨库改代码）

- 生产者 `capture_game_snapshot`（0x1417a1d00，country_core::systems::save::snapshot）：先构建 serde_json world state map（Walls/Roofs/Decorators/WallColors/GateColors/Terrain/Trees/Raster/Stairs 等段），随后调用 `Writer::add_chunk<&str>`（反编译行号约 1297，对应 batch-01 观察点 0x1417a31fa；batch-01 已记录共 3 处 add_chunk：固定 8 字节 `Snapshot`、动态资源段经 `tag_from_str_trunc` 后 u64 add_chunk、固定 7 字节 `Terrain`）。payload 经 `std::io::Write::write_all<ChunkWriter>` 写入（非有限浮点走 ryu 格式化路径）。add_chunk 失败时 `io::Error` → `anyhow::Error` 传播。
- 读取侧 `validate_and_load_closest_snapshot_data`（0x1417a6e50）：`Reader::iter()` → `impl$9::next()` 迭代 chunk（反编译行 206-208）；`try_load_snapshot_data` 内再次 `next()`（行 1367），随后 `flate2::mem::Decompress` + `DeflateDecoder` 解压（行 1377-1384），再按段 serde_json 反序列化。
- 完整动态 tag 生产/消费表、Snapshot/Terrain 每段 schema、压缩设置与迁移规则属 country-core save 系统范围，本轮不跨库实现，仅记录接线证据。

## 新增测试

- `byte_exact_header_layout_and_trailing_partial_tail_ignored`：写 `&str` tag chunk 后断言字节精确为 `[tag u64 LE][len u64 LE][payload]`（无 magic/padding）；Reader 忽略不足 16 字节的尾部残数据，位置停在残缺段起点。

## 依赖边界

byteorder 1.5.0、std IO 复用；serde_json/flate2/anyhow 为消费者依赖，非本库实现。未新增依赖，Cargo.lock 未变。
