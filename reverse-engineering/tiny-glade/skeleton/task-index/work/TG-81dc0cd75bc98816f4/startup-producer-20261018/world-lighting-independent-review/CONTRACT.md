# W35..W43 调用参数与 typed 输出独立审核

结论：**本段参数、返回值形体、history 有效位和 deferred 附件有界通过；首帧 SSGI 历史内容声明仍是实际接线阻断，整帧调用者尚待接入。** 本审查没有运行 Cargo/GPU，没有修改 production，也没有将本段 helper 或声明测试解释为完整世界渲染成功。

证据独立从 SHA `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03` 的 B 发行 RNE 按完整 PDB 范围提取。`original-manifest.json` 记录 `render_world 1404ec110` 的完整 4314 B SHA `520352bc4ed7343a5ada7f2f2e065ddcaa1e4914138a5778f3e88d65afe2fd81`，以及 half-res、RT GI、LightBinning、deferred、startup 与 temporal 原函数。callee 名来自原 PDB，不由当前函数名反推。

## 原调用值与当前映射

以下栈位置都是原 `render_world` 的 rbp 相对位置，不要求当前 Rust 复刻原 ABI。

| 序号 / 原 call | 原调用内容 | 当前 typed 映射 |
| --- | --- | --- |
| W35 `1404ec660 → 1408f6180` | GBuffer `{gbuffer, depth}`；返回两个 36 B 纹理：第一拷到 `+1e0`，第二到 `+160` | `HalfDepthNormals { depth, normals }`，未丢失第二图 |
| W36 `1404ec6d9 → 140a0d190` | full depth `+264`，velocity `+288`，此前 temporal depth `+b0`；返回 reprojection `+18c` | `gbuffer.depth, geometry.velocity, geometry.previous_depth` |
| W37 `1404ec708 → 1408743b0` | half depth `+1e0`，half normals `+160`，GBuffer `+240`，reprojection `+18c`；返回 SSGI `+8c` | `calculate_ssgi(&half, &gbuffer, &reprojection)`，host 按 depth/normals 顺序拆开 |
| W38 `1404ec986 → 140a01cf0` | GBuffer、half depth、half normals、history previous `+3b0`、valid bit、reprojection、SSGI | `ray_trace_gi` 保留所有输入与三张 `RestirTextures` 输出 |
| W39 `1404ec9ac → 140a44cd0` | half depth、half normals、SSGI、完整 RT GI 结果 `+340`、reprojection | `eval_gi_screen_radiance(&half, &ssgi, &ray_traced_gi, &reprojection)` |
| W40 `1404ec9c1 → 1409e7770` | rg/world；返回两个 RG buffer `+e0` | `LightBinning { grid, indices }`，未压成 texture 或单整数 |
| W41 `1404ec9e5 → 140a6fec0` | half depth、half normals、reprojection、W40 两个 buffers | `LightGrid { light_indirection_grid: bins.grid, grid_light_indices: bins.indices }`，count 从同一实际 Lantern owner 读取 |
| W42 `1404eca03 → 140a39e70` | GBuffer 与 reprojection；返回 screen shadows `-c` | `ScreenShadowInputs { a: gbuffer.gbuffer, b: gbuffer.depth, c: reprojection }` |
| W43 `1404eca67 → 1409d20b0` | 下表的全部值 | `DeferredInputs` 名字虽旧，实际值匹配原调用栈 |

原 RT GI 在 `140a03059..30ab` 分别写 sret `+0/+24/+48`；前面 `140a02e02/2e5a/2eaf` 的输出名 hash 是 radiance / ray_dir / reservoir，源局部量分别 `+c0/+90/+60`，返回次序一致。原 LightBinning 在非零实例分支 `1409e82f7..8318` 和零实例分支 `1409e8373..83e4` 都返回两个 24 B buffer；不能只保留第一项。

## W43 参数和附件

| 原参数 | 实际业务值 | 当前 DeferredInputs |
| --- | --- | --- |
| r8 / arg3 | `{GBuffer, full depth}` | `lighting[0/1]` |
| r9 / arg4 | scene color | 历史字段名 `depth` |
| stack +20 / arg5 | color history previous | `arg5` |
| stack +28 / arg6 | history valid 字节 | `arg6` |
| stack +30 / arg7 | SSGI | `arg7` |
| stack +38 / arg8 | half depth | `arg8` |
| stack +40 / arg9 | RT GI 三图 | 历史字段名 `gbuffer[0/1/2]` |
| stack +48 / arg10 | screen indirect radiance | `arg10` |
| stack +50 / arg11 | screen shadows | `arg11` |
| stack +58 / arg12 | ReSTIR direct-light reservoir | `arg12` |
| stack +60 / arg13 | light grid / light indices | `rt_buffers[0/1]` |

独立核对原 deferred `1409d2202..2298`：`arg3+24` 是只读深度和 viewport 来源，`arg4` 是 tag0 scene color LOAD 附件。当前 `ColorSpec::tag0(i.depth)`、`DepthSpec::ReadOnly { tex: i.lighting[1] }`、viewport `i.lighting[1]` 正确；不能被旧字段名字诱导为“把 depth 当颜色”或“RT GI 当 GBuffer 附件”。

四项原 startup call `140a97a76/ac2/b04/b45` 的顺序和目标 material slot 是 **1→3→0→2**（grass/canopy/solid/plant），每次成功后写该 slot，slot4 不写。当前 producer 保留这四个 URI/name/slot，实际 App 仍应由本轮 root 的 gameplay Startup 接线安装它；本合同不将单测创建的句柄当作真实启动产物。

## history 有效位与前置位置

- 原单张 temporal `1404f702e` 命中写 1，`1404f72d6` 新建写 0。
- 原 pair `1404f7605..760f` 合成两个单张历史的有效位，`1404f7696` 写成功结果尾字节。原 caller 在 `1404ec94c` 做 `&1` 传 W38，在 `1404eca08..47` 原样传 W43；没有取反。
- 当前字段虽然叫 `fresh`，其定义与真实 host 都表示 `history_valid()`。`history.fresh as u8` 正确；新分配应是 0，不能伪造 previous 内容或反转该位。
- 原 previous depth 建立在 `1404ec1f6..260`，早于第一个 W1 upload `1404ec2a5`。它继承 full depth 的 descriptor，仅覆写 format=100、usage=0x0c。
- 当前 `begin_depth_history` 用 canonical descriptor 设置 format/sample/storage 并显式清 attachment 两位，符合覆盖 usage 的原含义。它必须由整帧 caller 在 W1 之前调用，并把同一个 `TemporalTexture` 放进 `WorldGeometryOutputs.previous_depth`；W35..43 不可自行再分配一份。
- core 的记录型 `render_world` 也在 W1 前创建该历史。但当前 typed helper 的定义并不等于真实调用者已遵守这一顺序；本次 snapshot 未见真实整帧 `FramePasses` 实现或 `build_world_lighting` 的 production 调用。

## 仍会阻止首帧的具体缺口

**W37 新建 `ssgi_history` 后，p3 仍把 `pair.previous` 绑定成 active `0x19` 读。** 当前 temporal owner 在历史有效前不伪初始化；`ssgi.rs` 没有按 `pair.fresh` 设置该 binding 的 inactive-content 标记，当前严格 RG 会把这条读作为未初始化内容依赖。root 要求的下一步只对原 SSGI SPV 精确证明零 history_valid 分支不可采样后，补这一个 binding；不能凭 history_valid 名称统一跳过所有 history 读。

W38/39/42 与有灯笼时的 W41 也存在历史首次分配后的普通读声明；零灯笼时 W40/41 的原 placeholder 在 W43 上的内容需求需要原 shader gate 单独证明。这些是下一批有界调查点，本次没有代替原证据添加清零、inactive 或 preserving 标记。

上述缺口不改变 typed 参数审核结论，也说明“参数接对”仍不能声称本段真实首帧图通过，更不能声称完整 present 链已恢复。
