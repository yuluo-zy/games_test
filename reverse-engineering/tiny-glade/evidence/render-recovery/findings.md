# 渲染器 Rust 恢复：首批资源图规则

2026-10-12 用户提出按原 PDB 文件/函数骨架、Vulkan 公开 API、作者技术说明与原机器码恢复 Rust，并编译测试。本轮已经落实为可编译实现和原指令差分，完整渲染器继续保持未完成状态。

**路线可行；已取得的是源码恢复条件，不是自动还原能力。** Vulkan/ash API和已知公开库直接复用；PDB用于定位私有源文件、命名空间、实例和精确边界；反汇编/反编译用于恢复私有类型和规则；编译验证类型接线；原指令差分验证观察行为；最终GPU及图像验证另列。必须保持这些关口独立。

原构建为 tiny-glade.rne SHA256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`。原 PDB匹配，没有混用C/1.16构建、没有研究tiny-garden。

## 已实现并验证

| 私有函数 | 原VA / PDB字节数 | Rust实现 |
|---|---|---|
| ResourceInfo::link_up_next_access_chains | 0x140bd47d0 / 136 | resource_info.rs |
| ResourceInfo::calculate_lifetime_events | 0x140bd4860 / 359 | resource_info.rs |
| PassResourceAccessType::can_be_concurrent_with | 0x140bece70 / 84 | resource_info.rs |
| buffer_access_mask_to_usage_flags | 0x140bd4b00 / 50 | resource_usage.rs |
| image_access_mask_to_usage_flags | 0x140bd49d0 / 292 | resource_usage.rs |

- 访问链不清空next字段；重复previous按遍历顺序覆盖，越界前的写入保留。这些边界已用原码验证。
- 生命周期函数对tag恰好为1的首尾端点生成事件，将pass左移17位、资源索引直接OR，结束事件设置bit16。它会追加并排序已有事件；未额外添加清空、掩码或边界限制。
- 非写访问要求access类别相同；涉及写访问还要求非零且相同的同步组。`is_write`属于公开依赖vk-sync-fork，而不是游戏实现；Rust将classifier作为外部边界传入，没有重写依赖枚举或谓词。
- buffer usage按位组合；image usage仅接受12组完整mask，其他mask触发原panic。新实现保留触发规则，不声称恢复原panic文本和unwind ABI。

## 测试与依赖

`resource-info-verification.json`：78,867案例，包含所有256×256原分类字节组合、同步组边界、链的自引用/重复/前向引用/部分失败、生命周期tag/位打包/已有事件和超过16位资源索引。std排序逻辑在原机器码中执行；原CRT memcpy/memmove/memset通过实际宿主ucrtbase桥接。Vec预分配容量，不把分配器作为逆向内容。

`evidence/render-leaves/verification.json`：142,149案例，其中buffer 6,059、image 136,090。包括buffer十个有效输入位的所有1024组合、image低17位所有131072组合、32位边界和随机mask。**合计221,016案例全部通过**。

独立crate `reconstruction/render/Cargo.toml` 的12个registry依赖（含平台包）均与原锁文件版本、source和checksum匹配：ash `0.38.0+1.3.281`、libloading `0.8.3`、cfg-if `1.0.0`、windows-targets/platform `0.52.6`。保留主重建工程原Cargo.lock，使已有屋顶差分证明不因附加渲染依赖失效。

独立crate执行locked release build和 `cargo clippy --release --locked --all-targets -- -D warnings`；主工程重新locked release编译并通过6个CLI/瓦记录接线检查。没有以空单元测试计数作为行为验证。

复现：

```powershell
cargo build --release --locked --manifest-path D:/game/reverse-engineering/tiny-glade/reconstruction/render/Cargo.toml
& D:/game/reverse-engineering/tiny-glade/workspace/python/Scripts/python.exe D:/game/reverse-engineering/tiny-glade/scripts/render-recovery/verify_resource_info.py
& D:/game/reverse-engineering/tiny-glade/workspace/python/Scripts/python.exe D:/game/reverse-engineering/tiny-glade/scripts/render-leaf-verify.py
```

原函数体、PDB范围和RE-MCP结果分别在render-architecture、render-skeleton、render-leaves。原字节oracle与Rust输出比较记录位于resource-info-oracle.jsonl.gz。

## PDB骨架与难度

鲜读389个rhapsody/country_core编译单元；发现93个限定范围内的原文件路径及checksum，生成原层级的注释骨架。432个rhapsody和765个country_core::render native记录是私有命名空间定位信息，泛型/内联/别名仍需区分。

筛选到的4,124条native调试签名全部为0x1001 void()；TPI独立核实为空参数/void返回。因此不能从PDB直接生成正确的Rust函数签名和全部结构布局。未编译文件、优化掉的方法、编译单元范围外的pass/闭包也未被此93文件清单完整覆盖。注释骨架不计为可执行实现。

难度按已知证据评估：小型资源描述/访问规则容易分块；完整图编译/资源池/barrier/提交与回收中等至复杂，必须恢复跨对象不变量；GPU-driven剔除、GI/DoF/TGSR及与原画面一致更复杂，需恢复shader和CPU接口。公开API和作者思路显著降低研究成本，但具体句柄、同步、历史缓存和参数以原构建证据为准。

下一链为ResourceInfo::calculate完整入口 →资源注册/池→barrier生成→batch录制/提交/retire。原vk-sync-fork `0.4.0` 是无source的local项，公开最新版不等同此构建；其具体原来源/补丁范围需核实。当前只把classifier作为明确边界，不能宣称完整同步模块已连通。

## 公开一手参考与界限

- [ash原版本API](https://docs.rs/ash/0.38.0%2B1.3.281/ash/)：直接复用Vulkan绑定和输出usage类型。
- [Khronos同步指南](https://docs.vulkan.org/guide/latest/synchronization.html)：理解屏障、队列同步和验证层职责，不推断原作具体同步策略。
- [开发者官网](https://pouncelight.games/tiny-glade/info/)及[开发者受访回答](https://80.lv/articles/exclusive-tiny-glade-developers-discuss-bevy-proceduralism-publishers-cozy-games)：支持自建Vulkan/compute管线的动机，不替代函数体证据。

本轮没有创建Vulkan设备、提交GPU命令或运行原作画面；完整ResourceInfo、所有权/回收、多队列/帧在途、所有pass和shader算法尚未恢复。机器码校验针对局部规则与观察投影，不声称取得作者原Rust文本或完整原ABI。M12仅登记verified_partial。
