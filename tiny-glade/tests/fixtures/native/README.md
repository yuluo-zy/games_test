# 固定原侧夹具

本目录供 workspace 的 country-core、system-decorator、toolbar 测试通过 `include_bytes!` / `include_str!` 使用。生产路径不加载、校验或重建这些数据。旧 `reverse-engineering/tiny-glade/rounds/2026-10-20` 已删除，测试不再依赖它。

`provenance.json` 逐文件记录 SHA256、长度、来源与执行边界：

- preview-kind、decorator-extra、signifier-animation：从删除前保留的已编译测试程序常量区按地址恢复，原字节不变。这是历史捕获恢复，不宣称本轮重新执行原侧函数。
- photo-tree full/demo：从保留的 toolbar 测试程序常量区恢复。保留原 CRLF 与全部字节，树的标签、顺序、尺寸及 callback tag 不变。
- focus、appear：执行 B RNE 的原生可选组件分支、浮点算式与真实 easing 函数；查询迭代由解析后的组件存储代替，1848/66 个输入覆盖标记组合、custom kind、非有限值和原 scale。
- undo-auto-hide：3600 个帧输入，执行完整原函数及其时间、几何、动作、easing helper；只预置资源 wrapper、三个 anchor、足够容量的事件向量，无函数 hook。
- sounds：原指令执行实体代际/范围/组件 mask 检查、点击优先级和全部 drag byte 分支；输入事件迭代按原 focus→click→drag 顺序展开，预分配输出。578 个实体输入含旧代际/越界，73915 个输出。它不验证原版 ECS 调度或分配失败。

重建命令（从 `game/`）：

```sh
tools/venv/bin/python tiny-glade/tests/fixtures/native/rebuild.py
```

需要本地 `B版本内容/tiny-glade.rne`（脚本锁定其 SHA256）、Unicorn/pefile，固定捕获的种子文件已随项目保存并锁定哈希；历史测试产物尚在且指纹匹配时可再次精确提取，否则仅复用已校验种子。全新检出无需原测试产物即可使用固定夹具；种子缺失/变更则明确失败，不能用候选 Rust 输出替代。macOS 上 Unicorn JIT 需允许原生执行；本轮沙箱内执行失败，沙箱外采集成功。

新增/修改夹具须记录真实来源，按受影响测试重验；不得用待测生产实现生成其自己的 expected 值。编译或这些局部差分通过不代表完整游戏验收。
