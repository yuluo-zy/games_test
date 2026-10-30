# M01 装饰画面依赖：旗帜、灯笼、箭孔

本轮以 B 骨架已有 `crates/systems/decorator/src/decorator_visual/{flag,lantern,arrow_slit}.rs` 为唯一归属，没有创建新的业务目录。三者是既有 M01 画面依赖，不开展 M09 装饰物工具 UI。原参数证据复用 `140529c20-15生成器原B参数.json`；实际先列举并阅读了 B 的 `decorator_visual` 目录及三原文件。

## 原始输入与完成内容

| 原函数 | B 源 SHA256 | 实际 SystemParam | 本轮恢复 |
|---|---|---|---|
| `generate_flags@1407ff6c0` | `09e353aaade5769b9fa68f8e952ba9720a35420b5950f2cabe3dae1af50f5b33` | storage、PublicWalls、Derived、cached transform、blueprint、RaycastWorld | 11/12 墙旗与拐角旗、13/14 屋顶旗、15 地形支架与旗布全部写真实蓝图 |
| `generate_lanterns@14078d320` | `c773ab4a8dde605ad215753a99119e3d30b6359376726eab6f24862fc512b13e` | Derived、storage、cached、blueprint、PublicWalls、Query Roof、UiSignifierStream | 16 墙挂灯、17 地形灯与支架、18 平顶灯；真实平顶/地址门、局部灯参数、世界位姿、碰撞、描边 |
| `generate_arrow_slits@1407bbee0` | `6cf90892319dda7d481045d9c1cfbef261a8a87b20e5fc280804f39ac5345daf` | storage、Derived、PublicWalls、UiSignifierStream、blueprint | 原墙空间帽骨、三个 rank、22/23 分支、墙洞、玻璃与灯键、砖形变、半老虎窗洞记录 |

旧旗帜代码把 11/12 解释为“不产出”、15 解释为“另处处理”；原完整 body 明确包含它们的生成分支。旧箭孔所谓“四点剖面”实际是独立墙洞尺寸，`0x3f1ae148` 是原 `0.605000019`，不是旧 `0.6049`。以上错误接口和空消费已经删除。

墙旗保留墙顶插值、原墙空间的邻装饰距离、垂直重叠与顺序裁切、自由墙同朝向过滤、真实向下射线、最小布长与 NaN 选择规则，以及按实际布长生成的交互 cuboid。屋顶和地形旗的支架、鸟落点、材质、整数种子、布长载荷分别遵循原写面。几何不降成未经原证的四角布片。

三个生成器只发布 `Blueprint.transform`、`visual_entities`、碰撞及相应业务输出。实体仍由唯一 fullvisual 消费者创建、替换和回收，没有在生成器中建立第二套实体 owner。modified ID 先遍历原集合，再 required 查询存储；缺 ID 不再被 `filter_map` 吞掉。

## 灯光真实消费者

`maintain_lantern_lights@140790400` 恢复原八组输入：真实 `Query<(&mut LanternLightParams,&ColorId,Option<&SemiTransparent>)>`、SSBO 库、真实 GraphicsDevice、原 Local cache、音频 writer、最后一条 `LightLanternCmd(u32)`、ActiveSession、TimeOfDay。当前 Rust 安全宿主用 `ResMut<AssetSsboLibrary>` 取得已有 `InstanceBuffer<LanternSsbo>` 计数写权；不改变资产架构、不制造资源默认值。

原流程完整保留：按实际 decorator ID 初始化 RNG；从 TOD 的真实 `day_night_factor@+24` 与 olden 时间窗决定开关；预览强制关闭但显式点灯命令优先；收集真正点亮的查询实体；两次有序差分先释放旧槽再分配新槽；原 u16 空闲槽池、颜色/强度/世界位置比较、half 强度打包、GPU 脏区间上传及最高活槽计数。删除时只清除原槽的强度半字，不改位置和颜色，随后正确复用最低空闲槽。两遍的音效各自最多四条。

旧实现按 `storage.modified` 每帧重新随机生成假灯记录，既无真实组件查询，又混淆 28B 列表中颜色、亮度和位置偏移。该影子资源和错误布局已经移除。当前原布局为 ID@0、ColorId@4、强度@8、GPU u16 槽@12、位置@16。

原 builder `1412a9b30` 的灯位是 `translation + rotation * local_xyz`；旧代码漏世界平移。本轮已修。36B 灯组件统一为 core 的 `components/drawable.rs::LanternLightParams`，由 GPU owner 接真实 Nani emissive 读取，不使用地方 raw 组件。

## 绘制接线

原 builder 插入的地方 raw 旗组件与 draw callback 的独立 View TypeId 不同，属于确定的画面断链。本轮经中央授权，将墙旗与屋顶旗真实字段放到 core 原组件归属，builder 及 A/B 两套原回调只保留同 TypeId 的 alias，canonical 组件保留提取 journal 的 insert/remove hook。未修改渲染算法或另一个渲染引擎 agent 的实现。

物理 owner 对旗帜真实排墙射线补全了原 `legacy_raycast_w_params`：原 R9=1 是 `Some(WallId)`，不是 mask；原 mask 是 `0xf3 | additional_mask`，旗帜传 additional=0。完整查询的排墙 predicate 是 `0x105`，simple 是 `0x107`，不能互相替代。见 `m01-legacy-wall-ray-recovery-2026-10-27.md`。

## 有界原指令差分与验收

全部数值执行绑定同一原 RNE SHA256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`，未给数学过程设置替代桥接。

- `m01-flag-geometry-native.py/json`：48 组墙/拐角/地形位姿和 56 组高度帽，原指令逐位通过；包括非单位四元数、大坐标、负零和高度 NaN。高度帽恢复 MAXSS/MINSS 的第二操作数选择规则。
- `m01-lantern-slots-native.py/json`：264 组完整原槽 callback `141257c50` 逐位通过，覆盖新分配、相同 ID 更新、颜色/位置/亮度差分、NaN、±0、原软件 half 分支。边界明确为声音计数已达四、非空空闲槽；不声称执行了 GPU 驱动或整个 ECS 原函数。
- `m01-arrow-pose-native.py/json`：20 组原箭孔端点→四元数，原调用的 `140c9cdc0` 与完整 `140c911d0` 均实际执行，逐位通过。
- 真实 World 验收：五种旗帜完整生成→fullvisual→canonical 布组件→稳定跨帧→实体与碰撞回收；墙旗真实邻装饰裁切，以及排除本墙、命中其它墙的真实 Rapier 射线；三种灯及平顶/地形/楼梯失败门；箭孔所有合法 rank/子类型的墙洞、玻璃、形变和回收。数值差分不放宽精度。

本轮 CPU 验收与原差分不代表真实 GPU 场景验收。真实 SSBO 上传/计数、Nani 和 draw callback 的宿主联合验收由中央继续执行。`LightLanternCmd` 的真实颜色菜单 producer 属于 M10；当前只有事件合同和真实消费者，不能把自动夜灯测试说成点击闭环。

## 经验与需要继续跟踪的真实消费者

1. 不从反编译的跳转位置猜“另处处理”。完整 body 的标签可能被放在函数尾，两个跳过分支实际是最复杂的业务。
2. 先绑定原 SystemParam，再解读 Win64 ABI。Option tag、额外 mask、Vec3/四元数 sret 和内联 builder 字段都可能被 C 伪签名误导。
3. `builder`、`transform`、`visual_entities`、实体组件、extract 类型、draw callback 必须逐层核对同一 owner。编译通过和 builder 非空均不能证明画面可达。
4. 对反编译的算术括号再次核对 SSE。墙顶插值实际先各自完成两段插值再相加，不能按 C 的错误关联重排；NaN 必须核 MAXSS/MINSS 源操作数。
5. 原列表记录中的“看起来像 u64 的槽”不能直接命名颜色。颜色是 4B@4，后面的整型是 GPU 槽；原 typed consumer 与原排序比较共同消歧。
6. 灯维护输出 kind `0x0e` 的吞音缺口已由中央恢复为原 `Lantern/TurnOn`，并补真实 Studio 分支验收及原证据。仍须逐层区分维护事件、音频派发和实际宿主播放；不能以有事件冒充有声音。
7. 旗帜原会话字节最终由中央 fresh 核为 `SessionMeta.mutators`：Halloween2024/2025 的0/1触发专用纹理。中央已将三个旗回调统一读取真实 `ActiveSession.meta.is_halloween_any()`，删除地方 View 与每旗 Vec 克隆，并覆盖所有 glade×8 mutator组合。不能从旧 `glade_bytes` 变量名推导业务含义。

## 最终独占收据

`m01-three-owned-final-validation.json` 记录四条真实 cargo 命令、日志与源文件/manifest 前后 SHA；全部一致且通过：flag7、lantern6、arrow_slit2、共享 builder13，合计28项。正式 state0 和 preview state1 分别验证碰撞：原 `insert_for` 的第三参是 Info.state→u138，preview不注册正式碰撞；该参数不是interaction source flags。

广域 decorator_visual 最新联合轮次为229/231；两处其它 owner 的新 dormer/window 夹具失败另保留在联合日志中，没有抹成三family的全项目通过。生产GPU灯SSBO、旗帜实际draw/extract由中央独立联合验收，M10真实点灯按钮producer仍不能用夜间自动灯测试替代。
