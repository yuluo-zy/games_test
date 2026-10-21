# 完整楼梯插值候选独立源码审查

结论：通过本候选的有界源码审查，可与 XZ3275955f、已列六资源/请求 producer 依赖成组交 root 编译。此结论不等于测试已运行、C 验收或楼梯渲染宿主已经挂载。审查者 tree_commands 不是本源作者；未修改作者候选、生产、Cargo 或全局开关。

## 冻结身份与独立复现

- task TG-0ae36c51eb110c07c2，live owner core-duck-building-20261021 保持；本 review 写在其合法 work 的独立 review-tree 子目录。
- before `7eb0058346411ecfdbb1bd31851b83f2d376a841721f1ebc91f5d328820f2585`；after `67a1ed118b429f27d35e9cbe7b6aa0c64d94f372dd6f6251e60c8a00ec44b76f`；patch `3276eb28e09cc62f88973128909a1fb7bd3327a154e342760a70fcca240e3134`。
- 独立隔离复制 before，`patch -F0 -p1` 退出0，无 fuzz/offset，结果完整字节与 after 相等；见 SOURCE_REPRODUCTION_TREE.json。
- 原 PE f507a6ce 身份重算，主包全部1773指令/8622B连续逐字节相等；同验 six-resource caller228指令/1196B、zip120指令/381B、factor9指令/40B，以及356B segment fold。原 PDB module437:5700 原主函数8622B；module2:34676 wrapper1196B的完整 FunctionSystem 签名直接列出六资源顺序；见 SELECTED_PDB_IDENTITIES_TREE.json、NATIVE_BYTES_REVIEW_TREE.json。

## 完整控制流核对

1. 原 cf7dd..7ef 先 delta+28、rate9.0 调40B factor，再 cf82b..887 取旧 sampled owner、新 map 留 self；cf88c..8be 同 sub_segments 增量 extend。本候选不清独立旧段表和 arch_wall_supports。请求仍依 Vec 顺序，packed identity 与 actual segment 相同。
2. 原 cf9d0..fa35 是 XZ/height zip 的 min 长度 Vec3。cfa48..fb36 的四次 optional 查找分别 Info+28、extra+8、extra+10、extra+c；本候选保持同顺序，只先检查 raw disc<=1 才解析 Stair。缺 Info/非 Stair/未知 raw disc 返回明确原零支，不把未知 enum parser panic 带进 optional 分支。
3. cfc2a..fd94 从旧表 remove 一次后 clone old points 调真实 Curve3XZ；Err 走 fresh 并释放旧四Vec。oldCurve Ok 后 cff72..008f 的循环上限 min(point_count,u_count)、先 current positions index、再 current tangent index、再同数旧 points/tangents 或异数旧 get_coord 后四次索引，候选一致；没有两 Vec zip 截断、均匀比例或默认点。
4. cfe5a..cff6f 切向反号使用有序 dot<0，位置与切向按 old+(desired-old)*factor；倒长有限且>0才归一，否则 X。motion 采用尚未乘 factor 的 delta，按 (dx²+dy²)+dz² > 原5e-5，有序 NaN false；没有 clamp、后置实际位移阈值或 ZERO fallback。原 factor 自身也不另加 finite/范围门。
5. d00a0..026a 的 raw metadata/removed、width 长度、逐项 abs(diff)>原1e-4 的有序门，随后 canonical global lazy=true、same edge visual存在且 force=false，候选保持短路顺序。元数据用 u32 bits，保 NaN payload 和正负零；width NaN 仍不触发 ordered greater 分支。
6. d0756..0805 精确区分 old.points.len 与当前 positions.len：同数 clone old；异数才 move 当前已插值点。最终真实 Curve3/XYZ Try 失败写 None，旧 Option 的所有权由 Rust 赋值释放。d08af..0971 把整个 old 四Vec和metadata插回新表，候选不清旧 dirty、也不丢第四 opaque words。
7. d0330..05d8 fresh 顺序是 clone当前points/tangents/widths、第四Vec空、metadata、insert history，随后 d0679..cf960 输出 XYZ Try，失败也保 history/dirty=true。old 无效和全部失败门共享这条真实路径。被替换 value 及 loop 末 d0ac5 旧 map 未消费项由 RAII drop，对应原各三种元素大小和四Vec清理；不把旧 map 留在 owner，不仅 append。

## pinned API 与真实测试核对

当前 pinned Bevy0.16 `RunSystemOnce`、Resource/Res/ResMut、glam Vec2/Vec3、hashbrown map 的上述调用签名都存在；各 owner 的 public required APIs 和字段类型匹配。Rust 可分借 requests 与 previous 两字段，无本轮新增泛型/资源类型断点。全库 actual interpolate caller 搜索目前只有旧定义，本包新增真实系统测试，不存在仍传 old/new slice 的生产 caller 需要兼容；宿主未 mount 仍是显式边界。

10个新增测试都安装实际 ECS owners，通过实际 generate_playermade_stair_requests::<false> 从 storage/cache/graph/terrain 产生请求；seed_previous 由首帧实际 interpolate 发布历史，不 Default 一个 Params 或私建历史 owner。后续测试只显式改本案例所需旧缓存/元数据/输入。已审其断言分别覆盖：

- fresh 发布子段、消失 sampled identity 删除，独立 sub_segments/arch_wall_supports 保留；
- 同数 reuse 旧几何和四Vec、旧 dirty不被清；
- 异数按 old XZ u 重采并输出 current，但保 old 历史；
- old XZ非均匀重采、dt0时仍按未缩放 delta 标脏；
- tangent符号/零fallbackX、NaN width及严格 tolerance 边界；
- metadata各raw字段、NaN payload/负零与 removed 支持；
- 缺Info/非Stair/未知 raw extra 的零元数据；
- missing visual/force/global off各自进入fresh；
- NaN-Y oldXZ Ok、same-count old几何导致XYZ输出None，旧opaque和dirtyfalse保留；
- freshXYZ错误仍写含NaN原points的history并标脏。

global测试在本模块统一 Mutex + RAII恢复，所有消费用例均持锁；实际修改只存在本新增模块，不新增生产 writer。全局原 runtime writer 未锁定的边界不因本测试 store 而被冒称恢复。

XZ consumer case 必须配 TG9cab after3275955f；生产 snapshot 的 af00 会按全点finite gate拒绝旧 NaN-Y，导致应复用分支进入fresh。独立核候选 after 与冻结 metadata 相符，见 XZ_PAIRING_REVIEW_TREE.json。此新 case 能真正区别 shared ctor 修复前后，不是重复实现本系统算法。

## 验证范围

本次未 Cargo/rustc、未运行上述10测试、原目标或 GPU。未发现需要退回作者的新增源码/API断点；root 应配 XZ 增量先编译并执行真实测试，再记录实际结果。共享 get_coord 的此前极端 f32 域差异和上层 specialization/cache/完整装配器 U 仍独立登记；本审查未扩这些模块，也不声称把整个楼梯业务或渲染闭合。
