# SESSION-RENDER-U-CLOSURE：session-save serde 单态 + render 候选闭合（round3，2026-10-15）

任务：闭合 session-save 域 5 个 serde 单态（组A review 点名"从未反编译"）+ render 域 2 个候选
（`nani_calculate_draw_list_params` 空反编译、`collision::world::impl$41::into_transformed_shape`）。
标注：**O** = 机器码/反编译/常量证实；**I** = 推断（注明依据）；**U** = 卡点。
dbid `tg_us1`（新反编译落盘 `session-save/decomp/decomp-0x1403b5a20.txt`、
`decomp-0x141328d00.txt`、`decomp-0x1403b52d0.txt`、`decomp-0x1403b5500.txt`、
`decomp-0x141328bb0.txt`；collision helper 反编译/手解落盘 `collision-eco/decomp/`）。
**未改 reconstruction 源码。**

---

## 1. session serde 5 单态——**O（全部闭合：GladeType 主题 serde + 季节 mutator 枚举 serde）**

**判定总述**：5 个"SessionMeta serde 单态"反编译后实为**两个会话枚举的 serde 单态**：
（a）**GladeType**（会话主题，5 变体）——visit_str / deserialize\<Value\> / serialize\<Compact\> /
serialize\<Value\> 共 4 个；（b）**季节 mutator 枚举**（3 变体）——deserialize\<SliceRead\> 1 个。
功能证据（变体串匹配）指认；PDB `_::impl$0::*` 标签跨文件首个 derive 共用，不作类型判别。
这仍闭合 SessionMeta serde 合同的关键两环——`glade` 字段 JSON 形态（主题串）与
`mutators` 字段元素形态（季节串）；SessionMeta 字段集 {state, glade, mutators, image} 与
流式反序列化入口已在 `try_load_from_path` 0x140b46480（O）。

### 逐条

| va | PDB 名（SOURCE_MAP） | 判定 | 结论 |
|---|---|---|---|
| 0x1403b5a20 | impl$0::deserialize::impl$0::visit_str | **O** | **GladeType Deserialize 访问器 visit_str**：按串长 5/7/6 + 常量匹配 **{Summer=0, Olden=1, Flowery=2, Autumn=3, Winter=4}**，**大小写不敏感**（大写 0x65646c4f="Olde"+'n' 与 小写 0x65646c6f 均收）；未知串→ serde unknown_variant（FUN_1428a2480(str,len,&UNK_1429ccb38,5)，期望列表=主题名表） |
| 0x141328d00 | impl$0::deserialize\<Value\> | **O** | **GladeType 自 serde_json::Value 反序列化**（5 变体）：tag 3 = Value::String（串→变体索引，非 5 为成功，5=错误哨兵）与 tag 5 = Value::Object（hashbrown 查变体名，FUN_14143c450）两形；错误 "string or map"（@0x142c3a338/0x142c3a348） |
| 0x1403b52d0 | impl$0::deserialize\<Deserializer\<SliceRead\>\> | **O** | **季节 mutator 枚举（3 变体）流式 JSON 反序列化**：经 FUN_14005e900 匹配 **{Halloween2024=0, Halloween2025=1, Xmas2024=2}**（名串 @0x1429ccac6 len13 / @0x1429ccad3 len13 / @0x1429ccae0 len8；期望列表 @0x1429ccae8 3 项）；`"` 串形 + `{` map 形 + `:`/`}`/深度（+0x30 计数，0x15 溢出） |
| 0x1403b5500 | impl$0::serialize\<Serializer\<Vec\<u8\>,CompactFormatter\>\> | **O** | **GladeType 流式 JSON 序列化**：`switch(tag 0..4)` 写主题串（表 @0x1429ccb18="Summer"/@+6"Olden"/@+11"Flowery"/@+18"Autumn"/@+24"Winter"，len 6/5/7/6/6），经 FUN_1404b6100 写串；case 5+ 为 serde_json Serializer 内部跳表状态（Ghidra 坏指令告警，非业务） |
| 0x141328bb0 | impl$0::serialize\<value::ser::Serializer\> | **O** | **GladeType 序列化为 serde_json::Value**：按 tag 0..4 构造 **Value::String**（输出 `*p=3`=String 变体，+8 len、+0x10 ptr、+0x18 len；`func_0x000140613c10` 分配，内容同主题串表逐字节），分配失败 panic（FUN_1428d8fe3） |

### 会话枚举 serde 合同（存档 schema 最后两环，O）

- **GladeType**：`0=Summer, 1=Olden, 2=Flowery, 3=Autumn, 4=Winter`（5 变体）。
  - 纠错：WP7/WP13 恢复把 GladeType 还原为 `{Olden=1,Flowery=2,Autumn=3,Winter=4,Raw}` **缺 Summer=0 变体**——本轮 4 个单态反编译把值域钉死为 **0..=4 连续 5 变体**，`Raw(0)` 应为 `Summer`。
  - 与 `is_olden/is_flowery/is_autumn/is_winter`（0x140a9fa70/0x14092e6a0/0x14092e6c0/0x140ae8320 `cmp byte[rcx],1/2/3/4`）互证。
  - JSON 形态：**主题串**（写 `"Summer"`…`"Winter"`，两个 serializer 共用主题名表 @0x1429ccb18）；反序列化收**大小写不敏感**主题串与 `{"Variant":..}` 单键 map 两形。
- **季节 mutator 枚举（新名）**：`0=Halloween2024, 1=Halloween2025, 2=Xmas2024`（3 变体）。
  - 与 SessionMeta 季节谓词互证（DIGEST）：is_halloween2024=any(b==0)、is_halloween_any=any(b<=1)、is_xmas2024=any(b==2)——**"1=其他万圣节" 现钉死为 Halloween2025**。
  - 为该字节 Vec（SessionMeta 字段 @+0x08 ptr/+0x10 len，元素字节 0/1/2，O）的元素类型；与字段名 `mutators` 的对应标 **I**（字段名↔Vec 映射未独立核实）。
- **SessionMeta/ActiveSession 访问器 expecting（新证）**：0x140b47bb0 域三 expecting 小函数（手解 thunk，各 `lea rdx,[rip+..]; mov r8d,len; jmp fmt`）：
  - 0x140b47bb0 → `"variant identifier"`（len 18，@0x142aef5e8）= 枚举 visit_enum 的 variant-seed 访问器 expecting
  - 0x140b47bd0 → `"struct SessionMeta"`（len 18，@0x142aef5fa）= **SessionMeta 访问器 expecting**
  - 0x140b47bf0 → `"struct ActiveSession"`（len 20，@0x142aef60c）= **ActiveSession 访问器 expecting**
  - 即 session.rs 除两枚举外还有 SessionMeta/ActiveSession 结构体 serde 访问器；其字段名分发表仍在 try_load_from_path（首字节分发表 @0x142aef526）。
- **纠错/澄清**：组A review 称"SessionMeta serde 单态"——实为 **GladeType + 季节枚举**的单态（impl$0 标签在 PDB 提取中跨文件首个 derive 共用，不能作类型判别；visit_str/serialize 主题串 + FUN_14005e900 季节串 = 功能铁证）。闭合点仍是 SessionMeta 合同：`glade` 字段与 `mutators` 字段元素的 JSON 形态落定。

### 证据
- 5 个反编译：`session-save/decomp/decomp-0x1403b5a20.txt` / `-0x141328d00.txt` / `-0x1403b52d0.txt` / `-0x1403b5500.txt` / `-0x141328bb0.txt`（dbid tg_us1）。
- 关键 helper（本轮补）：`session-save/decomp/decomp-0x14005e900.txt`（季节枚举串匹配：len13/len13/len8 → 变体 0/1/2，期望列表 @0x1429ccae8 3 项，未知→ FUN_1428a2480）、`decomp-0x14005f940.txt`（`:` 后 "null" 解析，serde_json 内部）。
- 串表：rne-read `--hex 0x1429ccb18`（5 主题串）+ `0x1429ccb38`（期望列表 ptr）；`--hex 0x1429ccac0`（3 季节名 "Halloween2024"/"Halloween2025"/"Xmas2024"）；`--hex 0x142c3a300`（deserialize\<Value\> 错误 "string or map" @0x142c3a338）；expecting 三 thunk 手解 + `--hex 0x142aef5b0`（"variant identifier"/"struct SessionMeta"/"struct ActiveSession"）。
- xrefs：0x1403b5a20/0x1403b52d0/0x1403b5500 仅 DATA 引用自 0x143659388..0x1436593C4（连续 .rdata 函数指针表，派生 serde 分发表）。

## 2. `nani_calculate_draw_list_params` 0x1408baed0——**O（维持，闭合"空反编译"疑虑）**

- `render-top/decomp/decomp-0x1408baed0.txt`（518B）为 **Ghidra 跳表失败桩**：
  `/* WARNING: Could not recover jumptable at 0x0001408baf7d */` + 间接跳转经 `UNK_142aa15a8`
  分发表（`*(code*)(&UNK_142aa15a8 + _UNK_142aa15a8)(...)`）。函数体未被 Ghidra 还原，**不代表无业务**（PLAYBOOK §2 事实 10）。
- 结论维持 **O**，依据为**机器码展开深挖**（render-top 已落证）：循环体 4 份展开
  （0x1408bb0bc 系）；算法 `cmd_offsets[k*4+j]+=T[k]`、draws 钳制
  `!FORCE_FINE && (k>=2||FORCE_NO_FINE)→0x40`、`fine=draws>>5`（掩码 **0x3fe**）、
  `batch=(fine>>10)+(fine&0x3fe!=0)`；汇总 (3788800, 71936, 2048, 8)；`NaniDrawListParams` 0x70B。
- 纠错复核：与 `default_value` 0x1408bb650 仍是**两个独立函数**（0x1408baed0+0x77e /
  0x1408bb650+0x570 不重叠），不因反编译桩为空而回退。
- 章节已 O（`chapters/render/nani/buffers.rs.md`），本轮补 decomp 桩注记。

## 3. `impl$41::into_transformed_shape` 0x14155d340——**O（结构；字段语义 I）**

- **真名确认**：PDB（SOURCE_MAP）= `impl$41::into_transformed_shape`（行 2486）。
  **v1 误标 xz_flatten_aabb 排除**：真 xz_flatten_aabb = 0x1408d98a0（输出 4×f32
  {min.x,min.z,max.x,max.z}，collision-eco DIGEST 已证）。v2 无此符号 = 私有函数正常现象。
- **反编译结构（O，机器码逐字节核）**：0x28B=16+12+12 朝向盒 collider 变换输出：
  - `+0x00`：**Quat**（16B）= `from_rotation_arc(x 轴 → 足迹方向(p00,0,p04))`，
    helper `FUN_140d4ae30` = **两向量最短旋转四元数**（点积 + 叉积 + 归一化，反编译证实；
    glam `Quat::from_rotation_arc` 同型）。
  - `+0x10`：**Vec3 中心** {base_x, base_y, (min+max)/2}（竖直**中点**：机器码
    `movss xmm6,[rsp+0x28]` 在 c9cde0 写回 z=([p+0x18]+[p+0x1c])*0.5 **之后**读取）。
  - `+0x1c`：**Vec3 半尺寸** {size_x/2, size_y/2, (max-min)/2}（缩放常量全部 = **0.5**：
    `movsd xmm2,[rsp+0x20]; mulsd xmm2,[0x1429258e0]` 成对缩放 0x3f000000；`_UNK_142925870/
    _UNK_1429258e0/_UNK_1429258e4` 经 rne-read 逐字节读 = 0x3f000000）。
  - 输入 `param_2` 布局：`dir Vec2 @+0`、`base Vec2 @+8`、`size Vec2 @+0x10`、
    竖直 `{min @+0x18, max @+0x1c}`。
- **helper 手解（O，NotFound 小函数）**：
  - `func_0x000140c98660`（0x140c98660）：`out={in[+0x10], in[+0x14], in[+0x1c]-in[+0x18]}`。
  - `func_0x000140c9cde0`（0x140c9cde0）：`out={in[+0], in[+4], 第三参}`。
  - `func_0x000140c9cdc0`（0x140c9cdc0）：`out={in[0], 0.0, in[1]}`（xz 展平足迹方向）。
  - `FUN_140d4ae30`（0x140d4ae30）：from_rotation_arc（反编译已落盘
    `collision-eco/decomp/decomp-0x140d4ae30.txt`）。
- **判定**：结构 O（Quat 旋转 + Vec3 中心 + Vec3 半尺寸 = 0x28B）；`+0x10` 中心 z =
  竖直中点已由机器码读序证实。语义侧证：`new_cuboid`（0x1408cfff0）输入 half Vec3，
  本函数即由 {footprint, 竖直范围} 构造朝向盒的 {旋转, 中心, 半尺寸}（调用方接线未核对，
  标注接线 I）。
- 章节行已从 U 更新为 O（`chapters/systems/collision/world.rs.md` 行 81）。

---

## 交付
- 反编译：5 session 单态 → `session-save/decomp/`；`FUN_140d4ae30` → `collision-eco/decomp/`；
  三个 NotFound helper 手解见本文 §3（字节已读，不另落盘）。
- 章节更新：`chapters/resources/session.rs.md`（5 行 U→O + 实现方式 GladeType serde 合同 +
  胶水注记）、`chapters/systems/collision/world.rs.md`（impl$41 U→O）、
  `chapters/render/nani/buffers.rs.md`（decomp 桩注记）。
- 未改 reconstruction 源码。
