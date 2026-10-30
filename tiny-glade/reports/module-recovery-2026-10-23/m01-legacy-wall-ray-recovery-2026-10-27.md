# M01 旗帜完整射线的当前墙排除

已实际阅读原B骨架`crates/country-core/src/systems/collision/world.rs`的legacy、Rapier完整射线与两个predicate符号。只修改现world.rs原有查询路径，没有新增生产目录、复制raycaster或换成simple伪结果；旗帜生产者由rectangle owner修改。

## 原合同与本轮纠正

| 原符号/位置 | 机器码证据 | 本轮恢复 |
| --- | --- | --- |
| 旗帜`140800844` | R9=1，调用者stack+20=WallId，+28=0 | `Some(当前墙)`和额外mask0；`1`不是mask |
| `legacy_raycast_w_params@1408cc770` | `1408cc783 mov ecx,0xf3`，`1408cc788 or [entry_rsp+30]` | 基础mask `0xf3 | additional_mask`，不把额外位替换整个mask |
| `Rapier::raycast@1408cdc60` | 保存R9/stack墙ID为Option，0..16槽逐个查询；每次命中更新xmm6最大TOI | 复用实际cast_ray_and_get_normal、solid=true；最近命中收紧上限，等距后槽覆盖 |
| 完整predicate `1408cab80` | `1408cabcd mov r8d,0x105` | 只排除source kind0/2/8中同墙几何；kind1保留 |
| simple predicate `1408ca9a0` | `1408ca9ed mov r8d,0x107` | 保留MoveShapes恢复的kind0/1/2/8策略，不能直接替代完整路径 |
| generation检查 | predicate先取物理collider并验证mesh receipt，再读source和墙ID | 两种射线共用严格注册表读取和小型过滤helper，不跳过损坏receipt |
| legacy未命中 `1408cc7b7` | 原结果+30写整数2 | 安全语义结果仍保留`toi.to_bits()==2`，不写浮点2.0 |

准确旗帜调用在`140800844`，此前转述`1408007c4`落在SSE指令中段。已与rectangle owner纠正，其生产调用使用`additional_mask=0`和原方向`Vec3(+0,-1,-0)`。

公开接口为：

```rust
pub fn legacy_raycast_w_params_excluding_wall_id(
    &self,
    origin: Vec3,
    direction: Vec3,
    additional_mask: u32,
    wall: WallId,
) -> RaycastWorldResult;
```

原三参入口表达None，补齐其基础mask；新入口表达Some(wall)。统一完整实现`Rapier::raycast_with_params`，原raycast与显式max_toi入口都委托它。无native-physics的新业务入口明确失败，不能返回伪miss；该cfg与native实现互斥，不是重复方法定义。

## 独立原指令与非空物理场景

RNE SHA256：`f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`。

`m01_legacy_wall_ray_native.py`直接执行原完整/simple predicate：17种source、None/Some、相同/不同64位墙ID，另缺collider、缺注册表、越界索引、过期generation，合计144组含8组原panic。只桥接库ColliderSet查询与panic终止。另24组运行原legacy包装器，以明确的Rapier miss桥接捕获实际Option、mask、f32::MAX参数和legacy miss位型；不声称该桥接独立验证了Rapier几何内核。

结果保存在`m01-legacy-wall-ray-native.json`。原调用、wrapper、完整predicate、十七槽dispatcher分别保存为同名前缀`*-flag-call.asm`、`*-wrapper.asm`、`*-predicate.asm`、`*-dispatch.asm`。

真实`native-physics`测试不使用注入hit、空假场景或simple结果拼装：

- 68组原完整predicate正常输入分别建立真实Rapier世界，两块不同高度的实际cuboid。对原允许/拒绝结果核对来源、距离、位置与实际Y法线。
- 单独确认kind1完整射线仍命中当前墙，而simple会排除，避免两个0x10x位图混用。
- 原包装器24组mask输入驱动不同槽的真实近/远碰撞，验证基础位与额外位组合。
- 验证当前墙排除后最近命中、后槽更远不覆盖、等距后槽替换、剩余全被排除后legacy miss位型。
- 损坏receipt在墙排除之前失败；未被mask选择的槽不访问其坏receipt，加入该槽后按原校验失败。

## 验证结果

工作目录`/Users/yuluo/Desktop/game/tiny-glade`，共用原target，无额外构建目录：

```sh
/Users/yuluo/Desktop/game/tools/venv/bin/python reports/module-recovery-2026-10-23/m01_legacy_wall_ray_native.py
CARGO_INCREMENTAL=0 cargo check -p country-core --lib --features native-physics --offline -q
CARGO_INCREMENTAL=0 cargo test -p country-core --lib legacy --features native-physics --offline -q
CARGO_INCREMENTAL=0 cargo test -p country-core --lib systems::collision::world --features native-physics --offline -q
CARGO_INCREMENTAL=0 cargo check -p country-core --lib --offline -q
```

native-physics及不带feature构建均通过；legacy过滤13项全部通过；完整collision::world过滤51项全部通过。日志为`m01-legacy-wall-ray-tests.log`与`m01-legacy-wall-ray-world-tests.log`。定向diff检查通过。未进行OS输入、GPU画面或整游戏恢复验收。

## 经验与后续边界

1. Win64中R9之后的stack位移必须结合wrapper序言换算：此次Option标签被误读为mask，实际mask在再后一个参数。先核调用与被调入口两边，不能把C伪签名当源签名。
2. 几乎同体的完整/simple predicate仍有真实一位差异，0x105与0x107决定kind1是否保留。共享generation读取可复用，业务位图必须分别保留。
3. 只在最后命中时校验注册表不等价于原QueryFilter predicate；原先校验每个实际参与查询的候选，再决定墙排除，不能靠事后丢弃最近hit重跑或跳过坏receipt。
4. 返回现有安全`RaycastWorldResult`语义类型，不重新伪造原ABI结构；本轮没有扩散重写既有网格barycentric/UV转换族，旗帜使用的来源和命中位置已覆盖。
5. 全仓调用扫描确认旧三参legacy入口仅有空世界测试实际调用，其它出现为说明注释；旗帜使用新增typed入口。另`systems/wall-constructor/src/construct_elevation_supports/rectangle_pillars.rs`仍调用旧`raycast_simple_exclude_wall_id(SubworldColliderHandle)`兼容入口，该入口忽略旧错误形参。已交中央登记M06规则建筑核验，本次不越权扩展，也不据此宣称碰撞全部业务族完成。
