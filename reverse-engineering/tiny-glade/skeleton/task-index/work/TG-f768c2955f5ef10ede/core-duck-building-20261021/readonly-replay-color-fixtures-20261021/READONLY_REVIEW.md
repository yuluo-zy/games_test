# replay_color 两项实际 CPU 失败：fixture 位置错误

只读复核，未claim/修改任何core源，未运行Cargo。父agent当前要求仅给原证最窄候选方向。当前core新增adapter路径 `systems/history/replay_color.rs` 不在原mapped任务中；原源任务TG-8608a7dd158bc5f100对应 `systems/color/src/replay_color.rs`，live NULL，未领取。

## 实际失败与原因

brightness测试在读取 `as_lantern` 时panic，flag/window测试首先在读取`as_flag`时panic。公共测试helper `place(world,id,disc,extra_low)` 使用 `dst_payload = extra_low | disc<<56`。该字段属于 Info+0x10 destination，原Extra联合是独立 Info+0x30..44，niche在+0x41（u40的byte1）。因此实际extra仍为Default Stair0，Brightness分支不进入Lantern，Flag分支告警跳过；这不是disc数值变化，不应改生产匹配或增假fallback。

原32字节三From叶已按本RNE SHA f507a6ce独立读取，原PDB大小分别10/10/12：140839530 Flag写字节payload+0和niche+11=5；140839540 Window写u32和niche+11=6；140839550 Lantern写f32和niche+11=7。完整asm与PDB ID保存在本目录，未执行原target。当前canonical `set_extra` 按这些短叶写实际u30/u38/u3c/u40 owner，且不混入destination。

## 最窄修复方向（尚未实施）

保持replay生产原样，只修测试helper：签名改成 `place(world,id,extra:DecoratorExtraInfo)`，构造原有Info后 `info.set_extra(extra)`；三处调用分别typed `Lantern(LanternExtra(0.25))`、`Flag(FlagExtra(0))`、`Window(WindowExtra(0))`。不另写raw布局，不更换数字disc到另一个数值，不更改实际brightness/flag/window结果断言。也可保helper签名并在内部按这三个已证disc构typed variant，但typed参数能避免同类位置错误。

这两项旧失败只证明fixture不满足目标extra变体，不能证明adapter完整原色彩副作用已经恢复。该adapter头部已说明plaster/sound/SSBO等依旧partial，修fixture不能把这些登记为完成。生产 `apply_color_replay` 的Optional资源/警告策略本轮没有新增原证，仍不扩审或变更。

未运行修复后测试；root唯一负责应用合法委派候选与Cargo。若中央要实施，需先明确core adapter的现历史owner/legal work映射；本报告没有抢NULL原source任务。
