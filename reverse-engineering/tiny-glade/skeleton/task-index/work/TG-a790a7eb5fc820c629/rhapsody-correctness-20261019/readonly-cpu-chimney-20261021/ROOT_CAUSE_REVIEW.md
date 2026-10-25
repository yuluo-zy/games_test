# 三项 chimney CPU 失败的只读根因审计

本次仅定位 root 已执行的测试失败，未修改生产、候选、claim，未运行 Cargo。该源码由本 worker 先前恢复；这是失败排查，不冒充另一作者的独立业务审查。

## 实际失败点

root 日志 `/private/tmp/tg-canonical28-decorator-tests-20261021.log` 三项均在 `PublicWallState::max_y()`（public_wall_state.rs:205）失败，消息为 height cache 尚未计算。实际测试先共同调用 `chimney_world`，该 fixture 在 derive_decorator_subtype.rs:924 立即调用 `walls.get(wall_id).unwrap().max_y()`。所以本轮没有进入 derive 系统，更没有走到第二遍 mesh、strict distance、cache 分支。不可据此放宽生产 required max_y 或改邻近判定。

`public_wall(false)` 只创建 tag2 freehand WallState 并 `PublicWalls.consume`。`PublicWallState::collect` 尝试 regular cache，tag2 返回 UnsupportedShape，于是 regular/freeform 高度均 None。真正 freehand 高度由现有 `PublicWallState::prepare_paths` 的真实 path producer 再计算；fixture 漏了这一生命周期。

## 最窄修复建议（尚未写候选）

只在 `chimney_world` 将 walls 设为 mut，在读取 maximum 之前追加同文件已有真实 API 的初始化：

```rust
let (mut walls, wall_id) = public_wall(false);
let paths = PathRasters512::from_encoded(
    std::array::from_fn(|_| Arc::<[u8]>::from(vec![0; 512 * 512])), false,
).unwrap();
walls.get_mut(wall_id).unwrap().prepare_paths(&paths).unwrap();
let maximum = walls.get(wall_id).unwrap().max_y();
```

四个当前 path 栅格均为空是本测试场景输入；调用真实 prepare_paths 产生真实 prepared positions、segment owner 与 freeform cache，不手工填 maximum，不调用无原证 fallback。不要把此初始化放进所有 `public_wall` 测试 helper：其余测试正在验证 lazy/未访问输入，扩大初始化会改变那些测试的前提。

## 原第二遍与当前实现核对

完整原函数 140812b20/4576 B 位于 `before-frozen-20261021-canonical-id-producer/native-pack/per-frame-subtype-producer.{json,asm.txt}`；原身份沿 f507 RNE/2d968 PDB，PDB456:2900。现源码及完整原包哈希见 INPUTS_AND_FAILURES.json。

- 原 140813367/37a 只选 ty0x3b 且 dst<=1；1408133b6 required public wall，**1408133ca required max_y**，然后 required chimney extra。现生产顺序一致。
- span=(max(max_y-y,0)+3)+extension；同一原 WallSpace 投影 u；世界参考点位于 max_y，竖向中心位于 y+span/2。原 1408133f3..483 与现源码一致；无须改成烟囱中点或 outline 高度。
- 墙邻居原 140813604 subtype required→140813629 exact rank collision required→140813637 width(+30)/height(top-bottom)→676 strict vertical →688 同 WallSpace distance →69f strict horizontal →6ab 之后才排除 self。现 key(subtypes.get(other_id), other.rank)、bounds.footprint.size.x / top-bottom、strict门与排除顺序一致。
- roof 邻居原 140813572 真实 CachedDecoratorTransforms.position(other_id) required；sqrt(dz²+(dy²+dx²))<1.0，原5bd/seta保有序严格比较；5c6后才排除 self。现源码一致，无默认缓存。
- 同一 ID subtype map 第一遍 insert，第二遍命中20/无邻居19覆盖。邻居删除后重新 clear+derive，因此应回到19。直接 bucket fixture 不更新 d2s，但此 producer 全程只借桶迭代、subtypes 和明确 ID cache，不读 d2s；本轮不能归咎 d2s。

## 仅静态推导的修复后预期

本 fixture terrain decode 恒0，WallState::new 的 extra=4，空 path 栅格保 published y=0，真实 freeform producer 得 maximum=4，故 chimney span=7、half_span=center_y=3.5。

1. 墙邻居 y=3.5、collision width/height=2：vertical overlap 成立，u=0 距离0严格小于1+0.425→20；删除后19；u 恰为同 f32 阈值时 `<` 为假→19。
2. roof 缓存为真实参考点 + Z*0.5→20，+Z*1.0→严格边界19。
3. self collision 库清空时，第一遍 ID1=19 已写，第二遍 required key(19,1) 在 self 排除前 panic；存在 roof 邻居但无其 cache 时 required position panic。fixture 初始化修复后，这两条才真正得到验证。

以上为源码和原字节静态核对，不是当前测试已通过。由 root 应用最窄 fixture 修复后重跑这三项及必要完整组；本报告没有删测试、调整容差或修改生产算法。
