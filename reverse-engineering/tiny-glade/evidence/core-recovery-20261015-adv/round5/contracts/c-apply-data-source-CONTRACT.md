# Round5 C CONTRACT — apply 数据源接线（二进制节字节 → 逐域应用）

> 逆向 agent ① 交付。证据：session-save/ALGORITHMS §1.2/§2.7（load_game 快照加载流）、
> decomp-0x1417a6e50（1822 行解析器）、decomp-r5-0x1417a9c50（apply 读面）、crate snapshot.rs。
> 状态：**数据流与触发时序定案可直接落地；二进制节字节在解析结构的精确偏移待取证**。

---

## 1. 原生数据流（O，ALGORITHMS §1.2/§2.7 + 解析器/apply 读面）

```
快照 JSON 文件 → validate_and_load_closest_snapshot_data（0x1417a6e50，1822 行）
  └─ 逐节名分派解析（Version/Walls/Roofs/Decorators/WallColors/GateColors/Terrain/Trees/Raster）
     └─ 产出**解析后二进制结构**（0x1b0B 拷贝面）：
        {0xc0B 头部 + 11 域描述符 {ptr@X, len@X+0x10, count@X+0x1c}（基 0xc0/0xe0/…/0x240）}
        —— 每域描述符 **ptr 指向该节解析后的二进制记录数组**（Walls 0x78B 记录、
           Decorators 0x28B 记录、Terrain flate2 压缩字节、…）
load_game（0x14146d030，state 2 主流程）快照加载步：
  └─ 拷贝 0x1b0B 解析数据 → init_game_state_from_snapshot（0x1417a9c50）
     └─ 逐域描述符：读 ptr/len/count → 调 InnerWalls::load_state /
        DecoratorStorage::restore_snapshot / 逐域内联循环
```

- **二进制节字节来源** = 解析结构的域描述符 ptr（不是 JSON 原始串）。
- **触发时序** = `AppMode::LoadGame(0xe)` 门内 load_game_system → 快照加载完成 →
  `init_game_state_from_snapshot` → 之后 LoadingGameComplete/StartupRevealStarted 事件。

## 2. crate 对应（O + 待取证）

- 现 `apply_loaded_snapshot` 消费 `LoadedSnapshotData.loaded.sections`（**JSON 节**）+ 仅 log。
- 原生解析器（0x1417a6e50）产出**二进制节字节**（域描述符指向的记录数组）；
  crate JSON 路径 = 原生"节名→二进制记录"的**上游等价**（JSON 节载荷 → 反序列化 → 二进制）。
- **② 接线**：apply_loaded_snapshot 的 match 分支从 log::trace 改为：
  `parse_wall_section(json) → 0x78B 记录数组 → InnerWalls::load_state`、
  `parse_decorator_section(json) → 0x28B 记录数组 → DecoratorStorage::restore_snapshot`、
  `parse_terrain_section(json) → flate2 解压 → tg_terrain TerrainEditingData`。
- **待取证**：原生解析后二进制结构的**精确布局**（0x1b0B 内每域描述符槽位与
  ptr/len/count 的确切偏移；c-apply-api-formats/parsing 契约 §1-2 字段解码仍待证）——
  ② 可先按"JSON 节 → 本契约记录形状"落地，不等原生结构逐字节对齐（中优先）。

## 2b. 节数据形态定案（③ 追问，O：快照 schema = **JSON + deflate**）

- 快照文档 = **serde_json JSON**（crate snapshot.rs 头注：非 rkyv/ron）；节载荷 = **JSON 值**，
  **非纯二进制 blob、非 base64 整节**（Terrain 节例外，见下）。
- **Walls/Decorators 节**：JSON 值 = serde 序列化的墙/装饰状态；二进制 0x78B/0x28B 记录 =
  **JSON 反序列化后的内存形式**（load_state/restore_snapshot 消费），非节载荷本身。
- **Terrain 节**：deflate 压缩字节经 **JSON 字符串/数组承载**（`"decompressing terrain data"`
  @0x142cdc800；crate `as_str()→as_bytes()`/`as_array().first().as_str()` 双路径近似，
  native 精确编码待证）。
- **`{count@0, ptr@8}` = 内存 Vec（cap-first：cap@0/ptr@8/len@0x10）头部**，非节 JSON 载荷布局。
- **② 接线**：`parse_wall_section` 输入 = 节 JSON 反序列化后的二进制记录段
  （不是原始 JSON 串）；先 serde 反序列化再喂 load_state/restore_snapshot。

## 3. 可实现性
- **可直接落地**：数据流模型（§1）、触发时序（load_game state 2）、② parse→load 接线方向（§2）。
- **需再取证**：二进制结构逐槽偏移、0x78B/0x28B/Terrain 字段解码、X4 活羊 Query。

## 4. 证据索引
- session-save/ALGORITHMS.md §1.2（load_game 快照加载 + init_game_state_from_snapshot 0x1b0B）、
  §2.7（load_game state 2 主流程）。
- session-save/decomp/decomp-0x1417a6e50.txt（解析器 1822 行，产出 param_1 解析结构）。
- round5/decomp-r5-0x1417a9c50.txt（apply 读面：0xc0B 头 + 11 域描述符）。
- crate：systems/save/snapshot.rs:236-270（apply dispatch 现状）、resources/save_file.rs
  （LoadedSnapshotData/节）。
