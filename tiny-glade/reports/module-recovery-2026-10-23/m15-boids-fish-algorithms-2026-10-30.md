# M15 鱼与鸟的原版算法定点恢复（2026-10-30）

## 输入身份与证据

- B patched EXE `ljxsj_92385/Tiny Glade/tiny-glade.exe`，Ghidra worker `m15-animals-20261030` 回报 SHA-256 `cf048e27acdc266b2efc7c094dd435bacecb3f26c2afe54de4d09cd3f7036323`，见 `reverse-engineering/tiny-glade/workspace/m15-find-result.artifacts/000.stdout.txt`。该 worker 只读。
- 原版 `RAY_DIRS` 的 `Once` 调用点为 `0x141dc34e9`，初始化函数指针由 `0x142f7d660` 的首个 qword 指向 `0x141dc1fa0`。调用点与初始化主体由 `llvm-objdump -d --no-show-raw-insn --start-address=0x141dc1fa0 --stop-address=0x141dc2170` 核对。初始化循环 `0x141dc2050..0x141dc20da` 每次写 16 字节，共 `0x200` 字节。
- `0x142925904=0x3f800000`、`0x142ac4cc0=0x3d000000`、`0x142c8bb50=0xbf3504f3`、`0x142f7d5f0=0x4122a99b`，原字节见 `reverse-engineering/tiny-glade/workspace/m15-constants-result.artifacts/`。`0x140c904d0` 的 `lerp` 为 `a+(b-a)*t`。导入表和跳板确认 `0x142923e80=cosf`、`0x142923fd0=sinf`。
- 初始化顺序：`index=i as f32`，`t=(index+1)*1/32`，`z=lerp(1,-0.707106769,t)`，`radius=sqrt(min(1,max(0,1-z*z)))`，`angle=index*10.166407585`，写 `(radius*cosf(angle),radius*sinf(angle),z,0)`。`boids.rs` 据此取前三分量，去掉旧的黄金角近似和归一化。原版 16 字节槽的 `w=0` 未在当前 `Vec3` 表中存储，因为后续消费只读前三个浮点数；尚无原侧逐项浮点输出差分。
- `fish` 原反编译 `reverse-engineering/tiny-glade/evidence/toolchain/critters-extra/fish.decompiled.c` 第 159、620–638 行：`PathRastersResources::as_float_grid` 提供四张路径栅格，四次同点双线性采样按顺序相加，再 clamp `[0,1]`，小于 `0.1` 进入墙推开。`country-core/src/systems/paths.rs` 当前资源顺序为 path、cobblestone1、cobblestone2、cobblestone3。`fish.rs` 已删除四个水面偏移采样近似。
- 两处 boids 对齐权重原指令字节为 fish `0x3d6f9db2`（0.0585）、wren `0x3def9db2`（0.117）；随机锥上限 `0x3f7e76c9`（0.994）。此前源码十进制近似分别为 0.0584、0.1169、0.99376，现已改为位模式常量。
- 鱼出生在 `fish.decompiled.c` 第 181–276 行：候选索引用 `fastrand::Rng::usize(..len)` 的乘法高位及拒绝重抽逻辑（与本地锁定 `fastrand 1.8.0` 的 `gen_mod_u64` 相同），不能用整域随机数取余。原候选记录 +8 为 used 字节，水面采样 `<=0.5` 清零；在水中且 used 为零时置一并出生。现保留该标志及状态转移。
- 鱼出生随机顺序由原指令 `0x140f91054–0x140f911e0` 核对：先两次 RNG，x/z 分别加 `(u-0.5)*0.5`；`x0y@0x140c9cde0` 注入高度 `-0.8`，随后 y lane 加 `-0.25`，出生 y 为 `-1.05` 的 f32 运算结果；最后三次 RNG 生成三个速度分量，各减 `0.5`。`0x142a171f0` 两 lane 均为 `-0.5`，此前仅按反编译变量名误读成 y 速度 `-1.0` 的推断已由原字节排除。常量见 `reverse-engineering/tiny-glade/workspace/m15-fish-spawn-constants-result.artifacts/` 和 `m15-fish-height-result.artifacts/`。
- 原版鱼 boid 的 0x38 aux 槽包含目标、实体 ID（+0x10）、平滑方向、新方向和计时。`fish.decompiled.c` 第 297–325 行证实出生时顺序 push 0x18 Boid 和 0x38 aux；`BoidFlock_remove_fish.decompiled.c` 第 24–76 行证实移除索引倒序读取，两段同步 swap_remove。现已把 `FishBoids` 改成两段并行 Vec，并用同一个整数索引更新，不再按位置容差推测 self index。`despawn_fish` 直接重置两段资源。
- `fish.decompiled.c` 第 266–288 行的出生命令写入 0x30 B Fish 初始记录，状态槽清零；之前 `Fish {state:2}` 与原写点不符，现改为 0。原代码本帧出生后立即计入 flock 更新，本实现亦如此。
- 鱼新生 aux 初始化写点 `0x140f91229/0x140f91235` 读取 `0x142f7d768` 的 `(0,0,1)`；鹪鹩 `sim_wren_boids.decompiled.c.partial` 第 499–507、944–952 行写同一常量。两族的平滑方向和新方向均为 `Vec3::Z`，计时和目标激活为零。原字节见 `reverse-engineering/tiny-glade/workspace/m15-fish-state-result.artifacts/000.stdout.txt`，现已按该值初始化 `BoidState::default`。
- 羊抚摸动画 `sheep_pet_animation` 原 handler `0x14231afe4` 的 phase 0 只写控制标志和阶段。现移除了当前源码额外清零 `pet_progress` 的写入，保持原状态机写面；其余 handler 见既有 `sheep_pet_animation.rs` 的地址记录。
- 羊/鸭抚摸原函数 `142324f80.c`、`142322e30.c`：共同模式门 `Photomode` 需相机 mode 3、`Replay` 跳过、`Wall` 仅 Freeform 通过；相机 mode 3 距离分别 5/6。羊 widget 为 `pet_sheep{Sheep+0x20}`、球中心 `(x,y+0.8,z)`、半径 1；鸭 widget 为 `pet_duck{entity}`、球中心位置、半径 0.5。两者均使用空声音和不记录历史。现已在真实 `UiState` 注册和读取 drag 状态；羊 drag 3 对实际 `SheepAudio.elapsed>0.5` 调 `AudioInstance::start` 并清零，drag 4 写跟随状态 1 和 14.0；鸭 drag 3 写 frozen。
- `spawn_ducks.decompiled.c` 第 123–145 行：`if (0.99 < water) break; used=0;`，退出内层循环后仅 `used==0` 进入地形采样。旧实现误把 `water>0.99` 当作跳过并提前检查 used；现修为 `water<=0.99` 清 used 并跳过，深水且未用继续。排序比较器仍未还原。
- `fish.decompiled.c` 第 641–693 行显示墙段进入几何计算前先检查两个 `+0x20/+0x24` 高度均 `<0.2`，段长平方分母下限 `1e-20`，距离门为 `0x3f8f5c29`。首个命中段将鱼速度向 `normalize((pos−closest).xz)×|vel|` 插值，原指令 `0x140f91912–0x140f9192f` 显式传 rate `10.0` 和 dt；现已改为速度写面。墙空间索引的候选顺序及精确邻域仍待差分。
- `fish.decompiled.c` 第 771–812 行的第二实体查询会依序处理每个状态字节 2 且水平距离 `<2.0` 的 Fish 记录，目标速度为背离该实体位置的单位水平向量乘 `5.0`，`time_continuous_lerp_vec3` 的 rate 也为 `5.0`（原调用 `0x140f91cca–0x140f91ce7`）。旧代码错误地选择最近 Boid 并向其速度插值，现移除该假算法，保留独立 `repel_from_fish_targets` 算法。Fish 实体 0x30 位置记录尚未接入，当前主系统不调用该函数。

## 验证与边界

- 最终源码的 `cargo check -p system-critters --offline --quiet` 退出码 0；新增的真实 ECS 护栏测试 `cargo test -p system-critters --lib native_drag_phases_use_the_spawned_sheep_audio_and_follow_state --offline --quiet` 通过（1/1），覆盖 drag 3 音频门与 drag 4 跟随状态；暂存差异 `git diff --cached --check` 通过。构建包含工作区其他未提交改动，仅证明代码可编译且该交互链通过，不能证明原侧数值一致。
- `RAY_DIRS` 生成式和四路径采样数据源已按原版指令恢复；尚未用原程序逐项比较 `cosf/sinf` 舍入、栅格值及最终 boid 轨迹。
- 鱼群双 Vec 资源和移除算法已按原写点恢复；出生随机源种子、邻鱼/实体转向、光标和相机逃避、墙推开及速度档仍有未闭合或已标注近似。抚摸的 signifier、`WindowFocusEvent`、decorator 事件和鸭音频尚未恢复。羊、鸭、蝴蝶、萤火虫和鹪鹩的逐族算法、渲染、音频未完成原侧差分。以上均不得计作 M15 验收通过。
