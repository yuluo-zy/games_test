# 原 B 自适应四态与尺寸生成 V1（阶段一，不是宿主全链交付）

合法原TG-9da5 claim仍main-chain-boot持有，本目录是root明确委派窗口分辨率必要闭包。B/C源checksum不等，以下只限B。未改生产/Cargo/Git，无CPU或GPU执行；rustfmt只语法解析exit0。

完整主原证位于TG-c459/main-chain-boot-20261018/window-lifecycle-20261021/frame-resolution-recovery-v1/native-pack-v2。已固定B RNE/PDB、3 bounded requests、23符号点，collector实际3miss/4analysis+2version processes/1.396s，并非业务测试。初版请求最后采样点在边界外exit2失败保留native-pack/report.json；V2精确更正范围后exit0。不将shared decompiler漏jumptable后的return当空逻辑；所有下列状态按原连续机器码和两个真实4项跳表恢复，literal另绑定RESOLUTION_LITERALS.json。

## 四态

- 0 Initial（原startup唯一初始化状态）。
- 1 Waiting，持interval Duration及deadline Instant。
- 2 OutOfMemory，原render OOM只改tag=2；不读取其inactive时间载荷，预算回升不自动恢复。
- 3 Probing，恢复请求分辨率后持同一interval及稳定期deadline。

`current_scale > max_budget_scale`（原dccd2/jbe，NaN不进）时先发布current=max，再走1429ea498：0→Waiting(250ms,now+250ms)；1→同interval重新now+interval；2→保持2且此分支不读时钟；3→interval*=2，然后Waiting(now+interval)。先写scale后Duration/Instant溢出是可观察部分写，不能事务回滚。

否则先读Instant::now（原dcec0，包括tag0/2），再走1429ea488：0/2保持；1仅当now>=deadline且max_budget>=video.requested.unwrap_or(1)才先写current=requested、interval*=2、进入3(deadline=now+max(interval,2sec))；预算不足或未到deadline保持1；3到deadline恢复0，之前保持3。原branch细节分别dced8..dcf47、dcfa7..dcfdf、dcfe4..dd1dd。Duration乘法与Instant::add都复用std真实checked panic语义，不饱和、截断或造deadline。

注意：Initial状态不会因为直接修改GameConfig.video.world_resolution_scale就充当setter；真实启动FrameConfig用该请求值。OOM宿主必须读写FrameConfig.current，而不是改手动目标值GameConfig。这两处现工程适配将由阶段二替换，不能仅合资源类型称链路恢复。

## 原尺寸与部分提交顺序

1. 选TGSR比例：只有mode=1为preset.tgsr_resolution_scale，其他模式1；当前typedMode复用canonical值。写FrameConfig+1c。
2. `scaled_axis = extent_0_axis as f32 * current_scale`，饱和as u32再clamp[32,10912]，先写extent_8（dd32b）。
3. 提交选好的Adaptive四态（dd3b1..3c0）。
4. `inner = ((scaled_axis * tgsr_scale + 31.0) as u32 & !31).clamp(32,10912)`；必须先做未截断scaled乘法，不能先截extent_8再TGSR。真实literal1429e9600=31/1429e9610=~31。不是2对齐，不用ceil库改NaN/Inf路径。
5. 最终先extent_10（dd6a3）再extent_8（dd6ac）。两个float乘法和加法保持独立无FMA；Rust默认严格算术。

514x354、startup .75、TGSR Quality2/3实际结果：extent8=[385,265]，extent10=[288,192]。NaN/负/零映射32，正Inf映射10912；该数值helper只由原非零display门后调用，不能用32最小值吞掉OS零尺寸生命周期。

## 阶段二必须成套完成

- GameLoop原zero之后：真实Device.refresh_memory_budget→enabled时原vram::calculate_max_resolution_scale，disabled为f32MAX；当前sameDevice预算owner，不造预算/扩展支持。
- 当前swapchain旧extent和display不同则resize；纯状态方法需同一World的canonical FrameConfig/Adaptive和实际VideoSettings，borrow不能跨宿主调用。
- last inner Option与旧swapchainextent比较，必要ResourcePool.clear→两temporal新表→Device.wait_idle_and_recycle_resources→GpuLutTracker计数清零，最后才提交last inner。须保失败顺序/inflight owners；当前独立宿主等价实现另审。
- Resize原只写extent0；待本方法实际接线完成后才能同时移除extent8/10兼容简写。world_extent必须读已经生成的extent10而非WindowSize再乘GameConfig，防TGSR/预算失效和二次缩放。
- OOM host resolution_scale/set_scale/state必须canonical，不能保仅renderer.reduced_state私有数值或改用户目标。既有真实headless/warmup调用绕GameLoop，需要显式复用同一生成逻辑/owner并绑定验证，不把窗口通过外推它们。
- 7项tests是源写未执行，覆盖原真实basis、四态时间/回退、预算+deadline、sticky OOM、NaN/饱和/最小值、无伪setter及溢出部分写。root唯一执行CPU/GPU；此包没有宣布完整GPU/窗口质量链通过。
