# geometry 源码审查记录 · 2026-10-18

> 本报告为重构前的历史审查快照。当前实现已改变类型、上传和错误处理，旧诊断不得用于验收新代码；见 [后续重构报告](../geometry-refactor-2026-10-18/README.md)。

业务说明：[geometry/README.md](../../crates/country-core/src/geometry/README.md)。具体问题：[geometry/REVIEW.md](../../crates/country-core/src/geometry/REVIEW.md)。本轮新增文档与诊断证据，不修改五个被审查 Rust 源文件，也不更新原作恢复进度。

审查确认两个 P1：真实稀疏砖缓冲无法经当前 draw 实例视图取得句柄；墙砖 CPU 生产类型尚未连接到 SSBO 元素、分配和上传调度。其余输入限制和原实现已记载的 panic/载荷消费行为单列，未作为新恢复偏差修复。

## 实际验证

| 验证 | 结果 | 日志 |
|---|---|---|
| `cargo test -p country-core --lib geometry:: --locked --offline` | 28 通过、0 失败、0 忽略 | [lib-tests.log](lib-tests.log) |
| 临时 integration target，源码为 [probe.rs](probe.rs) | 6 通过、0 失败、0 忽略 | [probe.log](probe.log) |

两次 Cargo 均退出 0；存在模块外编译警告，未执行严格 Clippy、全库测试或 GPU 验收。首次测试曾等待共享构建目录锁，未停止其他构建。

诊断包含：

1. 球体 0/1/5/79 级的全部三角形外向绕序，以及位置、法线和 UV 的有限性。
2. 稀疏 chunk 的完整替换、缩减和显式空更新清除。
3. 未 update 的第二次上传清空旧几何。
4. 缺少 writer 的 panic 在 CPU 列表被消耗之后发生。
5. 容量耗尽时留下 2048 个已写实例、CPU 列表为空。
6. 真实 `BrickTransformSsbo` 类型化注册/读取成功，但经生产 `LibView` 查询得到 `None`。

这些是审查诊断，第 3–6 项验证当前限制或失败事实，不能把其通过理解为问题已经修复。CPU 设备只记录缓冲描述，不拥有物理 GPU 资源。生产类砖缓冲诊断仍创建了完整 192 MiB CPU 影子，其余上传诊断使用 64 chunk 的小型元素。

## 复现诊断

在 workspace 根运行以下 Python；临时测试文件使用独立名字，执行结束后清理，诊断源码保留在报告目录：

```sh
python3 - <<'PY'
from pathlib import Path
import subprocess
import uuid

root = Path.cwd()
source = root / 'reports/geometry-review-2026-10-18/probe.rs'
name = 'geometry_review_probe_' + uuid.uuid4().hex
target = root / 'crates/country-core/tests' / (name + '.rs')
try:
    with target.open('xb') as output:
        output.write(source.read_bytes())
    result = subprocess.run([
        'cargo', 'test', '-p', 'country-core', '--test', name,
        '--locked', '--offline',
    ])
finally:
    target.unlink(missing_ok=True)
raise SystemExit(result.returncode)
PY
```

## 来源与审查边界

本轮只进行当前 Rust 源码正确性审查，复用已有原作章节，没有重新反编译、修改 Ghidra/ReVa 数据库、运行原进程或比较原二进制输出。当前源文件的 SHA-256、命令与结果入口见 [verification.json](verification.json)。

开始审查时 `geometry/instanced_wall.rs` 与 `geometry/sphere.rs` 已有工作区修改，本轮保留。报告不借用旧差分结果宣布这些修改已经得到原作行为验证；也不将生成器和局部 buffer 测试等同于完整墙砖渲染验收。
