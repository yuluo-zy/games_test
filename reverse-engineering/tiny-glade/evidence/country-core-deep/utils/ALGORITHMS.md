# country-core utils 模块算法深挖（2026-10-15）

范围：`chapters/utils/**`（8 章 + 新增 float_grid 章）升级为算法级；本文件 = 深挖目标
（load_json / dirty_tiles / float_grid）全算法 + 共享库映射表 + 游戏自有 vs 共享边界 +
纠错清单。对照共享基础库 `reconstruction/shared-foundation/crates/utils`（tabloid 另见
`crates/tabloid`）。

---

## 1. load_json_as_mesh —— 全流程图（O，机器码为准）

**68 调用方**（startup 全部网格装载、mesh_library reload、logo/water/meadow…）依赖
country-core 自有实现；**共享 utils 没有此函数**（RENDER_GAP_ANALYSIS §4.1 确认）。

### 1.1 顶层流程（4 实例同体，仅输入转换不同）

```
load_json_as_mesh<T: AsRef<Path>>(path) -> Mesh
│
├─ 输入 → &str：<&str> 直用 {ptr,len}；<String> 解包 {cap,ptr,len}；<&PathBuf>/<PathBuf>
│   经 FUN_140d76ed0（Path::to_str）
│
├─ ① fs::read_to_string（FUN_140d59140，terrain-water DIGEST 同名证据）
│     Err → 格式化 panic "Couldn't find  {path}"（FUN_1428d9390，UNK_142ae80e0/80f0）
│
├─ ② 自定义 JSON 文本解析（FUN_141405930）→ Value 树
│     Err（tag 6）→ panic "Couldn't parse  {path}"（UNK_142ae8118/8128）
│
├─ ③ Mesh::new（FUN_140d61b00）：RandomState 密钥取线程局部（_UNK_142ac64c0..d8），
│    label = "load_json_as_mesh"（UNK_142ae7ffe，17 字符）
│
├─ ④ indices：
│    v = root["indices"]["buffer"]     // FUN_1414744d0 嵌套查键，字面量 "indicesbuffer"
│    v.tag != 4(Array) → panic（load_json.rs）
│    count = v.len；Vec<u32> 容量 count
│    逐元素（32 B 步长）：{tag@0==2(Number) && 子tag@8==0(PosInt)} 校验失败 → panic
│    （UNK_142c65a00）；u32 值取 @0x10 → 存入 Vec
│    SIMD 8-lane 求 max_index（argmax，`min` 语义展开 8 元素/轮 + 尾标量）
│    空数组 → max_index = 0xffffffff（bVar14=false 时）
│
├─ ⑤ attributes：
│    a = root["attributes"]（10 字符键 UNK_142ae8038）；a.tag != 4 → panic（UNK_142ae8048）
│    逐元素 {tag@0==3(String)} 校验（否则 panic UNK_142c65a18）→ 拷 {ptr,len} 名字对
│    └─ 每个属性：
│        r = read_attribute_from_json(a[i], name)      // §1.2
│        r.tag == 5（不识别）→ .unwrap() panic（FUN_1428d9760，"called `Result::unwrap()`
│           on an `Err` value"，payload = 格式化的 "load_json_as_mesh: unrecognized attrib
│           '{name}' with type '{type}'"）
│        assert count > max_index：count <= max_index → panic
│           "assertion failed: values.array_length() as i32 > max_index"（UNK_142ae8078，
│           load_json.rs:58）
│        Mesh::set_attribute(name, r.{tag,Vec}）       // FUN_140a35240，§1.3
│
├─ ⑥ 释放根 Value（tag 2 无动作 / 3 释放串 / 4 释放 Vec（FUN_140888d70）/ else 对象
│    （FUN_140898f40））；属性名数组释放
│
└─ 返回 Mesh 0x78 B：
     indices Vec<u32> cap-first @0x00
     source_file Option None（哨兵 0x8000000000000000）@0x18
     attributes HashMap<String,VertexAttributeValues,RandomState> @0x38（表 0x38..0x58
        + SipHash 密钥 0x58..0x68）
     label "load_json_as_mesh" @0x68
```

入口另有一条 puffin profile-scope 分支（`func_0x000140c16a00` = 全局字节 `*0x14362e880`
门控，U：写入者未找到）：scope 构造 + 单独解析路径后并入同一主流程（尾部多一次 Arc
释放，puVar27=1）。

### 1.2 read_attribute_from_json @0x14083e210（O）

```
输入 (attrs_obj, name) -> Result<{tag:u32, ptr, cap, len}, Err(String)>  （24 B 结构）

v = attrs_obj[name]（FUN_1414744d0，miss → 全局默认 *0x143630758）
t = v["type"]（键 "type"，UNK_142a93a12，4 字符）
type_str = format!("{}", t)（FUN_140d79f90 = fmt::format，serde_json-Value 形状节点的
   Display 渲染，如 ["int",1]）

匹配 type_str（按长度 9/11 + 前 8 字节常量比较）：
  ["int",1]   (9)  → FUN_141302940 → tag 1   // PosInt 子tag@8==0 校验
  ["float",1] (11) → FUN_1413011a0 → tag 1   // 允许 Float 子tag@8==1
  ["float",2] (11) → FUN_141302030 → tag 2
  ["float",3] (11) → FUN_1412fae80 → tag 3
  ["float",4] (11) → FUN_141476470 → tag 4   // Vec<f32> 直出（容量 count*4 元素）
  其它            → 格式化 "load_json_as_mesh: unrecognized attrib '{name}' with type
                     '{type}'"（UNK_142a93a50 起文案）→ Err（tag 5 传给调用方）
  长度 11 但 4 分支都不中 → unreachable!（invalidInstructionException）

数值解析（FUN_141302940 族，机器码 0x141302940 已手解）：
  源数组元素 32 B：{tag@0==2(Number), 子tag@8（0=PosInt/1=Float）, 载荷@0x10}
  迭代步长 32 B；f32/i32 载荷取自 @0x10；容量 = count*4 字节；结果 Vec {ptr@8,cap@0x10,
  len@0x18}（解析函数内 std 布局）
  解析失败 → panic（file load_json.rs line 0x1f=31，消息 "assertion failed: !v.is_empty()"
  @UNK_142ac6fdd / "incorrect attribute type" @UNK_142a93ab8 区）
```

说明：JSON 节点布局是 **serde_json-Value 形状但容器 cap-first**（数组 {cap@0x10,
ptr@0x10…} 实测：load_json 读 [0x10]=元素基址、[0x18]=len；read_attribute 读 [2]/[3] 同
位）——自定义解析器（非 serde_json，utils-input-config DIGEST 结论成立；serde_json-1.0.85
为 Cargo.lock 依赖但此解析器为游戏自有封装）。**"type 键 int/float tag 0..4" 修正**：
机器码产出 tag 1..4（1=float1/int1 → Float、2→Float2、3→Float3、4→Float4），无 tag 0；
tag 5 = 不识别错误。

### 1.3 Mesh::set_attribute @0x140a35240（O，2026-10-15 新反编译 12.6 KB）

```
hashbrown 查名：FUN_1405b1a70(param+0x58, name) = SipHash(name)
  SIMD 16 B 组比较（hashbrown Group）+ len 比较 + memcmp（FUN_142923bf0）
命中 → 按现有值判别元素字节宽（case 0/1→4、2→8、3→0xc、default→0x10）扩容 append：
      新数据 dealloc 旧 Vec；extend_from_slice 等价
未命中 → 新条目：{name String, 值 {tag,Vec}} 插入；值按 read_attribute tag 1..4 →
  VertexAttributeValues{Float=1,Float2=2,Float3=3,Float4=4}（Int=0 不产生）
```

---

## 2. dirty_tiles —— 位运算细节（O）

```
结构：bits: [u64; 64]（512 B = 4096 位）；set(x,y)：bits[y] |= 1 << (x & 0x3f)
world→cell：cell = (i32)trunc( fmaf(w, 32/65, 32.0) )   // fmaf = CRT 导入（jmp thunk
  @0x142923ef0 → IAT）；常量 @0x142925874：{32/65 = 0x3efc0fc1, 1/2600 = 0x39c9a634,
  32.0 = 0x42000000}
  语义：cell = (w + 65) / (65/32)；w ∈ [-65, +65] → cell ∈ [0, 64]
  有效性：cell < 0x40；越界/NaN → 哨兵 0x7fffffff（cell_from_world_space 返回
  Option 语义，iter 跳过）
  单元尺寸 = 65/32 = 2.03125 世界单位；地图范围 = 130×130（与 float_grid 的
  DEFAULT_WORLD_EXTENT=130、terrain_rock_mask 的 ±130 一致）
rasterize_world_space_stroke（@0x140ade620，28 调用方）：
  for 每段 (p1,p2)（Vec3 步长 12 B，x/z 两分量）：
    c1 = cell(p1.xz)；c2 = cell(p2.xz)
    r_cells = radius * (32/65) + 1/2600            // 半径 → 格 + 1/2600 ε
    FUN_140b1e460 = conservative_plot_line(c1, c2, r_cells, &self, &early_stop)
    // 复用游戏自有 crate conservative_line_raster（reconstruction reused.rs 有同名
    //  stub）；early_stop 置位则停止后续段（越出网格即停）
iter_world_space_rect_cells（@0x140ade8e0 等 7 实例）：
  4× fmaf 求 min/max 四角 cell（x 与 z 各两路）→ 双重循环 x∈[minx..maxx], y∈[miny..maxy]
  置位闭包：word[y] |= 1<<(x&0x3f)；越界 → panic（UNK_142ae2060 → dirty_tiles.rs）
  相交闭包：置栈 bool（返回 byte）
clear/new_clear：memset(self, 0, 0x200)（原字节 tail-jmp）
extend（@0x140960c30）：512 B 按 16 B 向量 OR（32× movups+orps）
tabloid 关系：
  DirtyTiles : State（empty=全零 + Clone=impl$4 glue @行13）
  DirtyTiles : StateDelta（apply_to @0x140960ef0 = 逐字 OR，16 B orps）
  Publisher::publish 按订阅者 pending 快照 OR 累积（共享 tabloid lib.rs 语义）；
  镜像测试 shared-foundation/crates/tabloid/tests/dirty_tiles.rs 已验证：快/慢订阅者
  的批次 OR 累积、sequence 语义与游戏一致
消费端（cross-domain）：wall 惰性更新 gather_lazy_update_sources（iter_tiles/
  iter_dirty_tiles ×2）、world_raster copy_to_staging（retain）、autoclutter ×4、
  stairs 传播（iter_tiles）
```

### ⚠️ 纠错（相对 utils-input-config DIGEST §2.1）

DIGEST 把 `rasterize_world_space_stroke` 常量 `{32/65, 1/2600, 32.0}` 读作
"单元 = 32 世界单位 → 64×32=2048×2048 地图"，**错误**。`32.0` 是 fmaf 的偏移
（cell 32 = 网格中心 = 65×32/65），不是单元尺寸；单元 = 65/32 = 2.03125，范围
±65（130×130）。本章节与 chapters/utils/dirty_tiles.rs.md 已按机器码修正。

---

## 3. float_grid —— 对照结论（机器码为准）

见 `chapters/utils/float_grid.rs.md`。要点：

- 游戏机器码 = 共享 utils 恢复版的双路径采样（`sampling_fix` ↔ 全局开关
  `*0x14362c308` = CENTERED_PATH，默认 false → 向量路径）；格点 u8/255、512×512、
  范围 130.0、lerp 顺序一致。
- 差异 1（API 形状）：自由函数 vs trait 方法——去重方向成立。
- 差异 2（NaN）：B 向量路径 NaN → 0；共享版 Rust `max/min` 保 NaN → frac=NaN →
  输出 NaN。**共享版需按 B 显式 NaN→0**（shared 注释已自认）。
- CENTERED_PATH 写入者已闭合（round3 `RENDER-U-CLOSURE.md` §2，O，机器码复验）：
  `load_game` 0x14146dc82-0x92：
  `mov eax,[rcx+0xc0]; cmp eax,0x1a; mov rax,[rip→0x14362c308(指针槽)]; setge byte [rax]`
  → `*0x143638fac` =（配置字段 @+0xc0 >= 0x1a）?1:0；同块 `setg byte [rip→0x14362c011]`
  （楼梯校验开关）共写——同一配置字段驱动两个全局（跨域线索）。配置字段 +0xc0 语义 U
  （0x1a=26 含义未定，候选存档版本/世界尺寸类参数，I）。`sampling_fix` 接线点 =
  load_game 调用点（共享 utils 的 `sampling_fix: bool` 参数按该字节传入）。

---

## 4. 共享库映射表（country-core ↔ shared-foundation/crates/utils）

| country-core 文件/函数 | 共享 utils 对应 | 状态 / 差异 |
|---|---|---|
| `utils/load_json.rs`（load_json_as_mesh ×4 + read_attribute_from_json） | **无** | 游戏自有（§1）；68 调用方依赖 |
| `utils/dirty_tiles.rs`（DirtyTiles 全部） | tabloid 协议（`crates/tabloid`）+ 镜像测试 | 位图本体游戏自有；State/StateDelta/Publisher 复用共享 tabloid；镜像测试已证语义 |
| `utils/debug_viz.rs`（Debug* 图元） | **无** | 游戏自有（调试命令队列） |
| `utils/geo.rs`（update_curve_mesh/rewire_enclosed_curve/mesh_from_pts） | geometry/curve 数学面（Aabb2/Rectangle2d/Circle2d/curve_slice/curve_u…） | 网格构建函数游戏自有；底层曲线类型共享已有等价 |
| `utils/mod.rs`（EventRegistry/world_space_to_screen_space） | **无** | 游戏自有 |
| `utils/audio_utils.rs`（AudioLoop） | **无**（kira 依赖复用） | 游戏自有 |
| `utils/project_cursor_onto_ray.rs` | camera.rs 不重叠 | 游戏自有 |
| `utils/child_wall.rs` | **无** | 游戏自有（wall 域） |
| `utils/float_grid.rs` | `float_grid.rs`（FloatGrid/FloatGridSampler/ByteFloatGrid） | 等价恢复；差异见 §3 |
| `utils/index_condenser.rs` | `index_condenser.rs` | 等价（共享版从同地址 remove 0x140a19f50 恢复，泛型 K；游戏版为 u32 实例 + key+4 稠密提示快路径；swap_remove + 被搬元素索引修复一致） |
| `src/name_hash.rs`（NameHash=xxh3_64） | `name_hash.rs` | 一致（均 xxh3_64 seed 0 默认 secret） |
| `utils/components/transform.rs` | `components/transform.rs` | 布局一致（{r@0, t@0x10, s@0x1c}；shared 注释同）；**shared 版 look_at/abs_diff_eq 已实现**（0x140c9ce00/0x140c9d0d0），country-core 版同名方法仍是 todo!（O 地址已标）——去重时以 shared 版为准 |
| `utils/curve.rs`（Curve/CurveU/crashy_*） | `curve/`（curve_slice, curve_u, curve_u_circular, resample） | 等价恢复面；country-core 侧多为 wp12 facade/typed gap |
| `utils/geometry.rs`（Circle2d/Rectangle2d/Aabb2） | `geometry/`（circle, rectangle, aabb, ray, plane, intersect…） | 等价恢复面（shared 有字段布局证据 {axis@0,center@8,size@16} 等） |
| `utils/rdp.rs` / `utils/swizzlers.rs` / `utils/comparative_space.rs` | `rdp.rs` / `swizzlers.rs` / `comparative_space.rs` | 等价恢复面 |

## 5. 游戏自有 vs 共享边界（结论）

- **游戏自有、共享无、必须留在 country-core**：load_json_as_mesh / read_attribute_from_json、
  DirtyTiles 位图本体、debug_viz 命令队列、geo 网格构建、EventRegistry、AudioLoop、
  ProportionalScreenSpace、ChildWall。
- **共享已有等价、可去重**：float_grid（先修 NaN）、index_condenser、NameHash、Transform、
  Curve/CurveU、geometry 数学类型、rdp/swizzlers/comparative_space（RENDER_GAP §7.1
  方向成立）。
- **依赖复用边界**：tabloid（共享 crate）提供发布订阅协议；conservative_line_raster 为
  游戏自有 crate（reused.rs stub，未恢复）；kira/glam/serde_json 为外部依赖不逆向。

## 6. 本轮新增证据

- `FUN_140a35240`（Mesh::set_attribute）新反编译 12.6 KB → `utils/decomp/decomp-0x140a35240.txt`
  （2026-10-15 从 /tmp/out-tgm3 落盘持久化；tg_m3 worker 原产出）。
- CENTERED_PATH 写入者闭合（RENDER-U-CLOSURE §2 复验，机器码见 §3）。
- `FUN_14083e210` 全部字符串常量解析（"type"/"buffer"/panic 文案/行号 31/58）。
- `read_attribute_from_json` 数值解析元素布局手解（32 B 步长、{tag@0,子tag@8,载荷@0x10}）。
- dirty_tiles world→cell 常量与 fmaf 语义、±65 范围确认（与 float_grid 130 一致）。
- `transform_screen_space` 结构补反编译（helper 常量未解析，维持 I）。
