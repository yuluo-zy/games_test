# utils + startup 未探究闭合记录（CLOSURE-ROUND3，2026-10-15）

任务来源：`round3/REVIEW-UNEXPLORED/groupE-newmodules.md`（组 E 清单 1、15、16 + §5 证据抽查
3 条 NotFound + 卡点复核）。范围：utils + startup 两模块。结论分级沿用 O/I/U。
本闭合作业为**文档与证据级修正**，不改 reconstruction 源码。

---

## 1. CENTERED_PATH 陈旧引用 → 闭合（组E #1，O）

- `utils/ALGORITHMS.md` §3 原「CENTERED_PATH 写入者仍未找到（RENDER_GAP §5.2 U 项维持）」→
  改为闭合（已更新 §3 + §6）。
- 闭合依据 = `round3/RENDER-U-CLOSURE.md` §2，并已机器码复验（rne-disasm，2026-10-15）：
  ```
  0x14146dc82  mov eax, [rcx+0xc0]          ; 配置字段 @+0xc0
  0x14146dc88  cmp eax, 0x1a                ; >= 0x1a(=26)?
  0x14146dc8b  mov rax, [rip+0x21be676]     ; 取 CENTERED_PATH 指针槽 0x14362c308
  0x14146dc92  setge byte [rax]             ; *0x143638fac = (字段 >= 0x1a)?1:0
  0x14146dc95  setg  byte [rip+0x21be375]   ; *0x14362c011 = (字段 > 0x1a)（楼梯校验开关同块共写）
  ```
- `chapters/utils/float_grid.rs.md` 已一致（写入者 = load_game 0x14146dc92、目标字节 0x143638fac、
  `sampling_fix` 接线点 = load_game 调用点），无需改动。
- 残余 U：配置字段 +0xc0 语义（0x1a 含义，候选存档版本/世界尺寸，I）——维持 RENDER-U-CLOSURE 登记。
- 注：utils `load_json_as_mesh` 入口另一全局字节 `*0x14362e880`（puffin profile-scope 门控）的写入者
  仍 U——与 CENTERED_PATH 是**两个不同全局**，本次不纳入闭合（无新证据）。

## 2. utils Mesh::set_attribute decomp 证据迁移（组E #15，O）

- `/tmp/out-tgm3/decomp-0x140a35240.txt`（12,959 B，tg_m3 worker 产出，内容为有效 `FUN_140a35240`
  反编译，非 NotFound）→ 已拷贝到持久路径 **`utils/decomp/decomp-0x140a35240.txt`**。
- `utils/ALGORITHMS.md` §6 首条改为引用持久路径；`chapters/utils/load_json.rs.md` 算法总览
  set_attribute 行补证据文件引用。
- `/tmp` 原文件保留未删（临时区）。

## 3. NotFound 坏证据修复（3 行，组E §5 证据抽查，O）

三行章节证据列原先都指向 `expansion-b2/decomp/decomp-0x*.txt` = `{"error":"No function at ...",
"database":"tg_b2"}`（Ghidra 未建函数 → NotFound）。全部改为机器码手解（rne-read --hex + capstone，
未占用 MCP 锁），证据落盘 `startup/decomp/`：

| 章节行 | 地址 | 手解结论 | 证据文件 |
|---|---|---|---|
| `chapters/startup/clearing.rs.md` `clearing::…::deserialize::visit_str` | 0x140a14620 | serde 枚举字符串反序列化胶水：长度分派跳表（len-6 ∈ [0,19]，表 0x142ac7dbc）+ 常量比较 → 判别。变体：`prefab`(6)→0、`placements`(10)→1、`particle_emitter`(16)→2（SIMD 16B 比较 @0x142ac70d0）、billboard 族(24)→3（双窗 SIMD：`billboard_expans`@0x142ac70c0 + `_expansion_scale`@0x142ac70b0）、未知→4。输出 {@0=0, @1=判别} | `startup/decomp/decomp-0x140a14620.txt` |
| `chapters/startup/init_photomode_state.rs.md` `PhotomodeState::on_enable_photomode` | 0x140870570 | 51B：`cmp byte[demo 0x143634d90],0; jne` → demo FOV=1.0（0x142925904=0x3f800000）/ 非 demo FOV=3.0（0x1429cd078=0x40400000）→ [rcx+0]=fov、[rcx+4]=-1、[rcx+8]=0、[rcx+0xa]=0、[rcx+0xc]=0、ret | `startup/decomp/decomp-0x140870570.txt` |
| `chapters/startup/mod.rs.md` `startup::impl$1::build` | 0x140b1ac90 | 8B：`mov rcx,rdx; jmp 0x140b65f80` 尾调用共享 build（0x140b65f80 = 插件注册/build 收尾，内含 Hash 0x8e1bd251962ba784 查表 FUN_1400dd6f0，同 add_game_startup_systems 尾部族） | `startup/decomp/decomp-0x140b1ac90.txt` |

`startup/ALGORITHMS.md` §7 证据落盘节已登记三条手解证据。

## 4. on_enable_photomode 0x140870570 完整验证（组E NotFound，O）

- 机器码复验（§3 表）：demo flag 分支、两个 FOV 常量位型（3.0=0x40400000 / 1.0=0x3f800000）确认；
  写序 [0]=fov / [4]=-1 / [8]=0 / [0xa]=0 / [0xc]=0 与章节算法节一致。
- **纠错**：章节与 ALGORITHMS 原记「邻接函数 0x140870590」实为错位——第二实例在 **0x1408705b0**
  （0x140870590 落在首函数 `mov dword[rcx+4],-1` 指令中部，非函数起点）。第二实例机器码：
  `mov rax,rcx; cmp demo; …; movss [rax]=fov; mov word[+0xc]=0x100; mov qword[+4]=-1; ret`
  （字段宽与首实例不同：qword[+4]、word[+0xc] → 另一结构/另一 impl）。已同步更正
  `init_photomode_state.rs.md` 与 `startup/ALGORITHMS.md` §6.8。
- demo 标志 0x143634d90 = RENDER-U-CLOSURE §1 的 demo 开关（.data 指针槽 → 目标字节），跨域一致。

## 5. startup 其余 U 复查（组E §2.3 复核 + 本次核对）

对 `startup/ALGORITHMS.md` 逐节核对，结论与组E §2.3「无大 U」一致。遗留/复核结果：

1. **28 系统逐名（§1.1）= U，维持**：Bevy 泛型 tuple（0x1c0B）不暴露名字；PDB 仅证 Audio 组 +
   `create_preset_textures`。单资产生产者章节已覆盖 16 个，其余属低优先（组E #11）。
2. **图集 parts key 常量语义（§2.4/§6.7）= U，维持**：stone_floor_=9、timber_=24、plaster=19、
   stairs_floor=26、smoke=21——疑固定数组索引/哈希，由 mesh_atlas 消费端索引模式裁决（组E #10）。
3. **spawn 记录表 24+ fn-ptr 族（§2.8/§5）= 族模式 O、个别 I（本次补验）**：capstone 复验 4 个
   （FUN_1412d7aa0/1412d7c70/1412d8b00/1412dc9f0）——每闭包 `add qword [r8], 增量`（表长递增，
   注记 (0x..B) = 增量+8，即含 8B fn-ptr 槽）+ 从 ctx 偏移 0x40/0x50/0x60/0x70 构建记录 + 调
   mesh-spawn helper（0x14205e980/0x140cf81b0/0x142058f60 等）。各类型身份仍按消费端上下文 I 级。
4. **`impl$1::build` 尾调目标 0x140b65f80**（§5 未登记，本次补）：= 插件 build 收尾 helper，
   内含 add_game_startup_systems 尾部的 Hash 0x8e1bd251962ba784 查表（FUN_1400dd6f0），
   与 §1.1 尾部 panic 同族——可入 §5 共享 helper 表（I）。

## 6. 未改动项 / 说明

- `reconstruction` 源码未改（本任务文档与证据 only）。
- 组E #16（components 45→44 计数）、#2/#3（asset_libraries 陈旧引用）、#4（PlacementDataSsbo
  schema）、#9（InputStream visit_str）、#10（atlas key）、#14（InstancedWall new/from）不属
  utils+startup 闭合范围，归各组/集成 owner。
- 本闭合未运行新 MCP 反编译（当时 .mcp-batch.lock 被其他 worker 占用；三个 NotFound 小函数
  均 < 60B，rne-read 手解充分，符合 PLAYBOOK §1.3 约定）。
