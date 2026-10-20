# 第 11 轮 DEC-C 交付：剩余装饰子系统（视觉状态 / 烟柱 / 资源）

> 实现 agent：DEC-C（第 11 轮）。目标 crate：`D:/game/tiny-glade/crates/systems/decorator`。
> 契约：`evidence/round11/contracts/dec-decorator-CONTRACT.md`（§2.4 #62 相邻面、
> §4.8 GPU 后置、§5.2 灯笼/窗灯与 §5.3 GPU 后置条目）。
> 文件权：仅 `chimney_smoke_anim_thickness.rs`、`decorator_resources.rs`、
> `decorator_visual_state.rs`；未改 `lib.rs` / `ui_systems/**` / stairs / lantern /
> 其余禁区文件；未改 country-core。

---

## 1. 交付摘要

| 文件 | 本轮内容 | 证据等级 |
|---|---|---|
| `src/chimney_smoke_anim_thickness.rs` | 重写为真实状态机：`HashMap<usize,f32>` 厚度表 + spot 收集输入、1.0/0 初值、`slice::contains` retain、逐 spot `init_rng` + 双分支目标/符号、`clamp01(old + sign*dt/target)`、spot `+8` 输出回写、查表失败 panic 路径；改为优先消费真实 `TimeOfDay`（U 回退镜像） | O 公式/结构；U 收集主循环与种子实参 |
| `src/decorator_resources.rs` | `DecoratorHandleName` 按 PDB 权威模块 re-export（类型身份不复制）；`impl$0::to_widget_name_hash` 落为方法 + 自由函数；算法由 xxh3 改为 O 的 widget-name 哈希（`WidgetId::from_name`，零键 SipHash13） | O（core 0x1405bdda0 注释）+ PDB 类型关系；U 无 |
| `src/decorator_visual_state.rs` | 补全持久/瞬时状态推进面：`is_sealed`/`len`/`is_empty`/`get`/`entities_of`/`sealed_ids`；seal/add/remove/drain 语义按 0x140811e80 / 0x1408119e0 / 0x140811c30 收紧；`DecoratorVisualState` 恢复行 114 `clone`/`eq` 合同 | O（三份 fresh 反编译）；`ClothInfo` 仍 U |
| `src/decorator_visual/mod.rs` | **未改**（本轮无新子模块；避免与 DEC②/DEC-B 冲突） | — |

测试：三文件内新增 20 个单元测试（见 §4）。

---

## 2. `chimney_smoke_anim_thickness`（0x140818df0）

### 2.1 已落地（O）

反编译全文：`evidence/restorer-state-decorator/decomp/0x140818df0_chimney_smoke_anim_thickness.c`。

1. **厚度表类型**：PDB `HashMap::retain<usize, f32, RandomState, Global, closure_env$0>`
   单态 + 表项 16 B → `ChimneySmokeThickness.thickness: HashMap<usize, f32>`
   （旧实现的 `HashMap<u64, ChimneySmokeEntry>` 已纠正）。
2. **帧初插入**（L383）：`entry.or_insert(if (t<0.72 || 0.9<=t) && spot.+0xd==1 {1.0} else {0.0})`
   落为 `initial_thickness(t, flag)`。
3. **retain 谓词**（L474 + PDB closure$0 的 `core::slice::impl$0::contains`）：
   只保留本帧 spot key；**删除旧实现 `thickness > 0.0` 的错误谓词**。
4. **逐 spot 演化**（L553-646）：
   - `init_rng` 每 spot 一次（O 调用点）；
   - 首抽无条件 `fastrand::f32()`；`+0xd==0` 丢弃首抽后再抽，`+0xd!=0` 用首抽；
   - 目标：`r*3+(1-r)` / `r*4+2*(1-r)`（3.0 @0x1429cd078、4.0 @0x142934694）；
   - 符号：`+0xd==0` 恒 `-1.0`；否则窗口内 `+1.0`、窗口外 `-1.0`
     （0.72 @0x142a80d44、0.9 @0x142a7c88c）；
   - `new = clamp01(old + sign*dt/target)`（L633-644，除数 = 目标值）；
   - 写回厚度表 + spot 记录 `+8` 输出（L646 `*(float *)(plVar37+1)`）；
   - 查表失败保留 `core::option::unwrap_failed` 等价 panic（`expect`）。
5. **时间源**：`TimeOfDay +0x08`（core `time_of_day`，L195 `*(f32*)(*in_R8+8)`）；
   `GameTime.delta`（core 注释 `delta @+0x28`，L633）。系统新增
   `Option<Res<TimeOfDay>>`，缺失时回退 crate 内 `TimeOfDayCycle.t`（lantern 镜像，
   保持既有注册兼容，U）。

### 2.2 仍 U（不得冒充）

- **spot 收集主循环**（in_RDX 墙 `+0xf8`/0xa0 段表、屋顶 0x48 段表）未实现：
  core 未暴露墙/屋顶密集表。以 `ChimneySmokeThickness.spots: Vec<SmokeSpot>`
  作为收集结果输入/输出面（`SmokeSpot{key, flag@+0xd, thickness@+8}`）。
  注：PDB 文件级候选把第二参写作 `Query<&mut Smoke>`（归属 unknown），
  但机器码 `in_RDX` 的读面与 PublicWalls/墙状态复合布局一致，按机器码建模。
- `init_rng` 实参寄存器未取到；以 spot key 作候选种子（`smoke_rng`），登记 U。
- spot 记录 0x10 B 的其余字段语义 U；GPU `update_buffer_bytes` 消费仍后置
  （契约 §5.3），CPU 状态与输出已真实。

### 2.3 测试

`smoke_is_active` 窗口边界（0.72 严格下界、0.9 闭区间）、初值 1.0/0 条件、
两目标公式与符号、`sign*dt/target` 除法与上下钳制、非 live key 即使厚度 > 0
也被 retain 删除、缺失 spot 先插初值后更新、flag=0 恒减薄并回写 spot、flag=1
窗口内增厚。

---

## 3. `decorator_resources`（PDB 模块 1499/3068）

### 3.1 已落地

PDB 该文件仅一个命名实体 + 一个 inline 函数（骨架
`skeleton/B/crates/systems/decorator/src/decorator_resources.rs`）：

- `DecoratorHandleName`：PDB 权威定义模块为本文件（IPI 3；
  `UiState::add<DecoratorHandleName>` 泛型关系）。实现以 **re-export**
  `crate::ui_systems::ui_decorator_affordance_dispatch::DecoratorHandleName`
  暴露同一类型身份，不复制定义（UI 族文件不在本 agent 文件权内）。
- `impl$0::to_widget_name_hash`（inline，源行 36；宿主
  `UiState::add<DecoratorHandleName>` 0x14125d180 /
  `create_decorator_widget<DecoratorHandleName>` 0x141282300）：
  - 落为 `impl DecoratorHandleName { fn to_widget_name_hash(self) -> u64 }`
    方法（对应 `impl$0`）+ 自由函数面；
  - 算法从旧的 `utils::name_hash::NameHash`（无种子 xxh3）纠正为
    `WidgetId::from_name`（core B 0x1405bdda0：零键 SipHasher13 + str 终止符，
    O 注释见 `country-core/src/systems/ui/resources/ui_state.rs`）。
- 其余“资源/事件/组件集合”：PDB 本模块无第二个命名实体；装饰域资源分属各自
  模块（`ChimneySmokeThickness` / `TimeOfDayCycle` / `DecoratorVisualState` 等），
  本轮不把无关资源搬入本文件，避免伪造归属。

### 3.2 测试

哈希与 core `WidgetId::from_name` 一致（3 个名字）、方法与自由函数一致且等于
`to_widget_id`、跨调用稳定且不同名不同值。

---

## 4. `decorator_visual_state`（可观察状态推进）

### 4.1 已落地（O）

证据：`groupD/decomp/decomp-0x1408119e0-visual_add_entity_1.txt`、
`decomp-0x140811c30-visual_add_entity_2.txt`、
`decomp-0x140811e80-visual_seal.txt`、骨架符号行号（17/29/35/40/46/49/58/71/76/90/97/114）。

- 持久表 `HashMap<u32, PersistentVisualEntities>`（值 `{sealed u8, Vec<Entity>}`）：
  - `add_decorator_visual_entity`：`or_default` → sealed `bit0` 检查 → push；
  - `seal_persistent_entities`：表空/查无 id 静默返回；`!=1` 写 1 返回；
    `==1` panic `"The persistent visuals are already sealed"`（0x142a8ecd0）；
  - `remove_persistent_entities`：HashMap::remove；
  - 恢复 `entities` 访问器（0x1408119d0 / 0x14067dd80）。
- 瞬时表 `HashMap<u32, Vec<Entity>>`（0x140811c30 机器码 8 B 元素，**不是**
  `DisplayChildAutoClutterInfo` 0x60 B 表）：`add` + `drain`。
- `DecoratorVisualState { persistent, transient }`：Clone/PartialEq（行 114）。
- 新增推进面（均由上述 O 语义直接导出，不引入新业务）：
  `PersistentVisualEntities::{is_sealed,len,is_empty}`、
  `DecoratorPersistentVisualState::{get,entities_of,sealed_ids}`、
  `DecoratorTransientVisualState::entities_of`、
  `DecoratorVisualState::{persistent_entities_of,transient_entities_of}`。

帧推进顺序（宿主 `lib.rs` 已接线）：`clear_out_visual_entities` drain 瞬时表 →
生成族 add 持久实体 → 按 id seal；持久表**不在帧清系统清除**（reviewer-state
2026-10-17 复核）。

### 4.2 仍 U

- PDB 命名实体 `ClothInfo`（本模块定义，被 `generate_clothline` 的
  `Box<ClothInfo>`/`Option<Box<ClothInfo>>`/`Iter<ClothInfo>` 消费）字段布局
  unknown；`decomp-0x1407e5190` 未保留类型名。本轮**不编造字段**，登记 U；
  接入需 `generate_clothline.rs`（不在文件权内）与布局取证。
- sealed 字段精确宽度（seal 8 B 写 / add 读 bit0 / seal 判 `!=1`）：以 `u8`
  承载已观察值域，宽度差异不影响判别（登记）。

### 4.3 测试

持久 add/seal 正常路径、seal 后 add panic、重复 seal panic、未知 id seal 静默、
remove 后重建恢复未封存、`sealed_ids` 过滤、瞬时 add/drain 且不影响持久表、
空 drain、Clone/PartialEq 合同。

---

## 5. 编译与测试

- 编译（最终源码）：`CARGO_TARGET_DIR=/d/game/tiny-glade/target-agent-decc cargo check -p system-decorator --offline`
  → **通过**（`Finished dev profile in 9m 39s`；仅既存 warning，本文件无新 error）。
- 测试：`CARGO_TARGET_DIR=/d/game/tiny-glade/target-agent-decc cargo test -p system-decorator --lib --offline <filter>`
  （测试二进制 `target-agent-decc/debug/deps/system_decorator-7a8dcf6b6a4fab2a.exe`；
  多 agent 共享构建期锁较长，最终过滤结果如下）。

### 5.1 结果

| 过滤器 | 结果 |
|---|---|
| `chimney_smoke_anim_thickness` | 8 passed; 0 failed |
| `decorator_resources` | 3 passed; 0 failed |
| `decorator_visual_state` | 9 passed; 0 failed（含 2 个 `should_panic`） |

合计 20 passed。`cargo check` 在测试后的最终注释/措辞修订版本上复跑通过。

---

## 6. 未闭合 / 跨 crate（不由 DEC-C 修改）

1. `ChimneySmokeThickness` 的 spot 收集主循环依赖 core 未暴露的 PublicWalls/屋顶
   密集表（`+0xf8/+0x100/+0x108/+0x110`、0xa0/0x48 段表）；需 core 提供只读收集
   面或在 wall/roof crate 提供适配器后接回 `SmokeSpot`。
2. `Query<&mut Smoke>` / `GlobalSmokeControl` 在 core 中无定义（PDB 候选签名）；
   本轮以 crate 本地 `ChimneySmokeThickness` 承载已取证的厚度表语义。
3. 烟柱 GPU `update_buffer_bytes`（rhapsody 设备 owner）与 SSBO 布局仍后置。
4. `ClothInfo` 布局与 `generate_clothline` 接线（见 §4.2）。
5. `DecoratorHandleName` 的 UI 族实现（`create_decorator_widget` 的 flags/sounds
   载荷）仍归 `ui_systems/**`，不在本轮文件权内。
